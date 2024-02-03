use std::sync::Arc;

use common::crds::{Agent as K8sAgent, AgentSpec, AgentStatus, Manager};
use common::models::FactionSymbol;
use eyre::Result;
use kube::api::{ListParams, Patch, PatchParams};
use kube::core::ObjectMeta;
use kube::{Api, Resource, ResourceExt};
use tracing::info;

use crate::ship::patch_ship;

pub(crate) async fn reconcile(manager: Arc<Manager>) -> Result<()> {
    info!("reconciling agent");

    reconcile_token(manager.clone()).await?;
    reconcile_starting_ships(manager.clone()).await?;

    info!("agent reconciled");
    Ok(())
}

async fn reconcile_token(manager: Arc<Manager>) -> Result<()> {
    info!("reconciling token");

    let agent = get_agent(manager).await?;

    if let None = agent.spec.token.clone() {
        let token = register_agent(agent.spec.symbol.clone(), agent.spec.faction.clone()).await?;

        let spec = AgentSpec {
            token: Some(token),
            ..agent.spec.clone()
        };

        let new_agent = K8sAgent::new(agent.name_any().as_str(), spec);

        let serverside = PatchParams::apply("operator");
        let ns = agent.namespace().unwrap();
        let client = crate::get_client().await?;
        let agent_api: Api<K8sAgent> = Api::namespaced(client, ns.as_str());
        agent_api
            .patch(
                agent.name_any().as_str(),
                &serverside,
                &Patch::Apply(new_agent),
            )
            .await?;
    }

    info!("token reconciled");
    Ok(())
}

async fn reconcile_starting_ships(manager: Arc<Manager>) -> Result<()> {
    info!("reconciling starting ships");
    let agent = get_agent(manager.clone()).await?;

    let symbol = agent.spec.symbol.clone().to_uppercase();
    let command_ship = format!("{}-1", symbol);
    patch_ship(manager.clone(), command_ship).await?;

    let satellite = format!("{}-2", symbol);
    patch_ship(manager.clone(), satellite).await?;

    info!("starting ships reconciled");
    Ok(())
}

pub(crate) async fn get_agent(manager: Arc<Manager>) -> Result<K8sAgent> {
    let client = crate::get_client().await.unwrap();
    let agent_api: Api<K8sAgent> = Api::namespaced(client, manager.spec.namespace.clone().as_str());
    let lp = ListParams::default();
    let agents = agent_api.list(&lp).await?.items;
    match &agents[..] {
        [a] => Ok(a.to_owned()),
        [_, ..] => return Err(eyre::eyre!("multiple agents found")),
        [] => return Err(eyre::eyre!("agent not found")),
    }
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

async fn register_agent(symbol: String, faction: FactionSymbol) -> Result<String> {
    let conf = openapi::apis::configuration::Configuration::new();
    let req = openapi::models::RegisterRequest::new(faction.into(), symbol);
    let res = openapi::apis::default_api::register(&conf, Some(req)).await?;

    Ok(res.data.token)
}
