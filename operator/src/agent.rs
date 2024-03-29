use std::sync::Arc;
use std::time::Duration;

use common::crds::{Agent as K8sAgent, AgentSpec, AgentStatus, Manager};
use common::models::FactionSymbol;
use futures::StreamExt;
use kube::api::{Patch, PatchParams};
use kube::core::ObjectMeta;
use kube::runtime::controller::Action;
use kube::runtime::Controller;
use kube::{Api, Client, Resource, ResourceExt};
use openapi::apis::configuration::Configuration;
use snafu::{ResultExt, Snafu};
use tracing::{info, warn};

use crate::ship::patch_ship;

pub struct AgentControllerData {
    pub api_config: crate::ApiConfig,
    pub k8s_client: Client,
}

impl AgentControllerData {
    pub fn new(k8s_client: Client, api_config: crate::ApiConfig) -> Self {
        Self {
            api_config,
            k8s_client,
        }
    }
}

#[derive(Debug, Snafu)]
pub enum AgentError {
    #[snafu(display("error listing agents {}", source))]
    ListAgentError { source: kube::Error },

    #[snafu(display("expected 1 agent but found 0"))]
    AgentNotFoundError,

    #[snafu(display("expected 1 agent but found {}", num_agents))]
    TooManyAgentsError { num_agents: usize },

    #[snafu(display("error patching agent {}", source))]
    AgentPatchError { source: kube::Error },

    #[snafu(display("error patching ship {}", source))]
    ShipPatchError { source: eyre::Report },

    #[snafu(display("error registering agent {}", source))]
    RegisterAgentError { source: eyre::Report },
}

pub(crate) async fn run_controller(data: Arc<AgentControllerData>) -> eyre::Result<()> {
    let agent = Api::<K8sAgent>::all(data.k8s_client.clone());

    Controller::new(agent.clone(), Default::default())
        .run(reconcile, error_policy, data)
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(())
}

pub(crate) fn error_policy(
    _object: Arc<K8sAgent>,
    err: &AgentError,
    _ctx: Arc<AgentControllerData>,
) -> Action {
    warn!("{}", err);
    Action::requeue(Duration::from_secs(5))
}

pub(crate) async fn reconcile(
    agent: Arc<K8sAgent>,
    ctx: Arc<AgentControllerData>,
) -> Result<Action, AgentError> {
    info!("reconciling agent");

    reconcile_token(agent.clone(), ctx.clone()).await?;
    reconcile_starting_ships(agent.clone(), ctx.clone()).await?;

    info!("agent reconciled");
    Ok(Action::await_change())
}

async fn reconcile_token(
    agent: Arc<K8sAgent>,
    data: Arc<AgentControllerData>,
) -> Result<(), AgentError> {
    info!("reconciling token");

    let authed_config = match agent.spec.token.clone() {
        None => {
            let token = register_agent(agent.spec.symbol.clone(), agent.spec.faction.clone())
                .await
                .context(RegisterAgentSnafu)?;

            let spec = AgentSpec {
                token: Some(token.clone()),
                ..agent.spec.clone()
            };

            let new_agent = K8sAgent::new(agent.name_any().as_str(), spec);

            let serverside = PatchParams::apply("operator");
            let ns = agent.namespace().unwrap_or("default".to_string());
            let agent_api: Api<K8sAgent> = Api::namespaced(data.k8s_client.clone(), ns.as_str());
            agent_api
                .patch(
                    agent.name_any().as_str(),
                    &serverside,
                    &Patch::Apply(new_agent),
                )
                .await
                .context(AgentPatchSnafu)?;

            get_authenticated_config(token)
        }

        Some(token) => get_authenticated_config(token),
    };

    let mut cfg = data.api_config.write().await;
    *cfg = Some(authed_config);
    info!("token reconciled");
    Ok(())
}

fn get_authenticated_config(bearer_token: String) -> Configuration {
    Configuration {
        bearer_access_token: Some(bearer_token),
        ..Default::default()
    }
}

async fn reconcile_starting_ships(
    agent: Arc<K8sAgent>,
    data: Arc<AgentControllerData>,
) -> Result<(), AgentError> {
    info!("reconciling starting ships");
    let oref = agent.controller_owner_ref(&());

    let symbol = agent.spec.symbol.clone().to_uppercase();
    let command_ship = format!("{}-1", symbol);
    patch_ship(
        data.k8s_client.clone(),
        command_ship,
        agent.namespace(),
        oref.clone(),
    )
    .await
    .context(ShipPatchSnafu)?;

    let satellite = format!("{}-2", symbol);
    patch_ship(data.k8s_client.clone(), satellite, agent.namespace(), oref)
        .await
        .context(ShipPatchSnafu)?;

    info!("starting ships reconciled");
    Ok(())
}

pub(crate) fn create_owned_agent(source: Arc<Manager>) -> K8sAgent {
    let oref = source.controller_owner_ref(&()).unwrap();
    let spec = AgentSpec {
        symbol: source.spec.symbol.clone(),
        faction: source.spec.faction.clone(),
        token: None,
        reset_date: None,
    };

    K8sAgent {
        metadata: ObjectMeta {
            namespace: Some(source.spec.namespace.clone()),
            name: source.metadata.name.clone(),
            owner_references: Some(vec![oref]),
            ..Default::default()
        },
        spec,
        status: Some(AgentStatus {
            checksum: "".to_string(),
            ships_initialized: false,
            last_updated: None,
        }),
    }
}

async fn register_agent(symbol: String, faction: FactionSymbol) -> eyre::Result<String> {
    let conf = Configuration::new();
    let req = openapi::models::RegisterRequest::new(faction.into(), symbol);
    let res = openapi::apis::default_api::register(&conf, Some(req)).await?;

    Ok(res.data.token)
}
