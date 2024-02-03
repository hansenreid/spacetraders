use common::{
    crds::{Agent as K8sAgent, Manager, ManagerSpec},
    models::FactionSymbol,
};
use futures::StreamExt;
use kube::{
    api::{ListParams, Patch, PatchParams, PostParams},
    runtime::{controller::Action, Controller},
    Api, Client, ResourceExt,
};
use snafu::{ResultExt, Snafu};
use std::{sync::Arc, time::Duration};
use tracing::info;

use crate::{agent::create_owned_agent, create_namespace};

#[derive(Debug, Snafu)]
pub enum ManagerError {
    #[snafu(display("error listing agents {}", source))]
    ListAgentError { source: kube::Error },

    #[snafu(display("expected 1 agent but found {}", num_agents))]
    TooManyAgentsError { num_agents: usize },

    #[snafu(display("error patching agent {}", source))]
    AgentPatchError { source: kube::Error },

    #[snafu(display("error patching agent status {}", source))]
    AgentStatusPatchError { source: kube::Error },
}

pub(crate) async fn run_controller(client: Client) -> Result<Action, ManagerError> {
    let manager = Api::<Manager>::all(client);

    Controller::new(manager.clone(), Default::default())
        .run(reconcile, error_policy, Arc::new(()))
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(Action::await_change())
}

pub(crate) fn error_policy(_object: Arc<Manager>, _err: &ManagerError, _ctx: Arc<()>) -> Action {
    Action::requeue(Duration::from_secs(5))
}

pub(crate) async fn reconcile(
    manager: Arc<Manager>,
    _ctx: Arc<()>,
) -> Result<Action, ManagerError> {
    info!("Reconciling manager");
    let serverside = PatchParams::apply("operator");
    let client = crate::get_client().await.unwrap();
    let agent_api: Api<K8sAgent> = Api::namespaced(client, manager.spec.namespace.clone().as_str());

    let lp = ListParams::default();
    let agents = agent_api.list(&lp).await.context(ListAgentSnafu)?.items;
    let new_agent = create_owned_agent(manager.clone());
    let agent = match &agents[..] {
        [_] => return Ok(Action::await_change()),
        [] => &new_agent,
        [_, ..] => {
            return TooManyAgentsSnafu {
                num_agents: (agents.len()),
            }
            .fail()
        }
    };

    agent_api
        .patch(
            manager.name_any().as_str(),
            &serverside,
            &Patch::Apply(&agent),
        )
        .await
        .context(AgentPatchSnafu)?;

    agent_api
        .patch_status(
            manager.name_any().as_str(),
            &serverside,
            &Patch::Apply(&agent),
        )
        .await
        .context(AgentStatusPatchSnafu)?;

    info!("manager {} reconciled", agent.name_any());
    Ok(Action::await_change())
}

pub async fn init_manager(symbol: String, faction: FactionSymbol) -> eyre::Result<()> {
    let client = crate::get_client().await?;

    let name = symbol.clone().to_lowercase();
    let namespace = format!("spacetraders-{}", name);
    create_namespace(namespace.as_str()).await?;

    let manager_api: Api<Manager> = Api::namespaced(client, namespace.as_str());
    let manager_spec = ManagerSpec {
        symbol: symbol.clone(),
        namespace,
        faction,
    };

    let manager = Manager::new(name.as_str(), manager_spec);
    manager_api.create(&PostParams::default(), &manager).await?;
    info!("manager for {} created ", symbol);

    Ok(())
}
