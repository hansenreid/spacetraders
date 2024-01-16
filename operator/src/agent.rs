use std::sync::Arc;

use common::crds::{Agent as K8sAgent, AgentSpec, AgentStatus, Manager};
use common::models::FactionSymbol;
use eyre::Result;
use kube::api::{ListParams, Patch, PatchParams};
use kube::core::ObjectMeta;
use kube::{Api, Resource, ResourceExt};
use serde_json::json;
use tracing::info;

pub(crate) async fn reconcile_agent(manager: Arc<Manager>) -> Result<()> {
    info!("Reconciling agent");

    let serverside = PatchParams::apply("operator");
    let client = crate::get_client().await.unwrap();
    let agent_api: Api<K8sAgent> = Api::namespaced(client, manager.spec.namespace.clone().as_str());

    let lp = ListParams::default();
    let agents = agent_api.list(&lp).await?.items;
    let new_agent = create_owned_agent(manager.clone());
    let agent = match &agents[..] {
        [_] => return Ok(()),
        [] => &new_agent,
        [_, ..] => return Err(eyre::eyre!("multiple agents found")),
    };

    agent_api
        .patch(
            manager.name_any().as_str(),
            &serverside,
            &Patch::Apply(&agent),
        )
        .await?;

    agent_api
        .patch_status(
            manager.name_any().as_str(),
            &serverside,
            &Patch::Apply(&agent),
        )
        .await?;

    info!("agent {} reconciled", agent.name_any());
    Ok(())
}

pub(crate) async fn reconcile_token(manager: Arc<Manager>) -> Result<()> {
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

pub(crate) async fn patch_status(manager: Arc<Manager>, agent_status: &AgentStatus) -> Result<()> {
    let client = crate::get_client().await.unwrap();
    let agent_api: Api<K8sAgent> = Api::namespaced(client, manager.spec.namespace.clone().as_str());

    let status = json!({
        "status": agent_status
    });

    agent_api
        .patch_status(
            manager.name_any().as_str(),
            &PatchParams::default(),
            &Patch::Merge(&status),
        )
        .await?;

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

fn create_owned_agent(source: Arc<Manager>) -> K8sAgent {
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
