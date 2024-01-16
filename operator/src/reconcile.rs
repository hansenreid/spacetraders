use crate::agent::{reconcile_agent, reconcile_token};
use crate::ship::reconcile_starting_ships;

use common::crds::{Manager, ManagerSpec};
use common::models::FactionSymbol;
use eyre::Result;
use k8s_openapi::api::core::v1::Namespace;
use kube::api::PostParams;
use kube::core::ObjectMeta;
use kube::runtime::controller::Action;
use kube::Api;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

pub(crate) async fn reconcile(obj: Arc<Manager>, _ctx: Arc<()>) -> Result<Action, crate::Error> {
    reconcile_agent(obj.clone()).await.unwrap();
    reconcile_token(obj.clone()).await.unwrap();
    reconcile_starting_ships(obj.clone()).await.unwrap();

    Ok(Action::await_change())
}

pub(crate) fn error_policy(_object: Arc<Manager>, _err: &crate::Error, _ctx: Arc<()>) -> Action {
    Action::requeue(Duration::from_secs(5))
}

pub async fn init_manager(symbol: String, faction: FactionSymbol) -> Result<()> {
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

async fn create_namespace(name: &str) -> Result<()> {
    let ns = Namespace {
        metadata: ObjectMeta {
            namespace: Some(name.into()),
            name: Some(name.into()),
            ..Default::default()
        },
        spec: None,
        status: None,
    };

    let client = crate::get_client().await?;
    let ns_api: Api<Namespace> = Api::all(client);
    ns_api.create(&PostParams::default(), &ns).await?;

    Ok(())
}
