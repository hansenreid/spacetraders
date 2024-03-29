use std::sync::Arc;

use common::crds::Manager;
use eyre::{Context, Ok, Result};
use k8s_openapi::api::core::v1::Namespace;
use kube::api::PostParams;
use kube::core::ObjectMeta;
use kube::{Api, Client, Config, CustomResourceExt};
use openapi::apis::configuration::Configuration;
use tokio::sync::RwLock;
use tokio::task::JoinSet;
use tracing::info;

mod agent;
mod manager;
mod ship;

pub use manager::init_manager;

use crate::agent::AgentControllerData;
use crate::ship::ShipControllerData;

type ApiConfig = Arc<RwLock<Option<Configuration>>>;

async fn get_client() -> Result<Client> {
    let config = Config::infer().await?;
    Client::try_from(config).wrap_err("error creating k8s client")
}

pub async fn run() -> Result<()> {
    info!("Running operator");
    let client = get_client().await?;
    let api_config = Arc::new(RwLock::new(None));
    let agent_data = AgentControllerData::new(client.clone(), api_config.clone());
    let ship_data = ShipControllerData::new(client.clone(), api_config.clone());

    let mut set = JoinSet::new();

    set.spawn(manager::run_controller(client.clone()));
    set.spawn(agent::run_controller(Arc::new(agent_data)));
    set.spawn(ship::run_controller(Arc::new(ship_data)));

    while let Some(result) = set.join_next().await {
        let _ = result?;
    }

    Ok(())
}

pub fn crdgen() -> Result<()> {
    let manager_crd = Manager::crd();
    let manager_yaml = serde_yaml::to_string(&manager_crd)?;
    print!("{}", manager_yaml);
    println!("---");

    let agent_crd = common::crds::Agent::crd();
    let agent_yaml = serde_yaml::to_string(&agent_crd)?;

    println!("{}", agent_yaml);
    println!("---");

    let ship_crd = common::crds::Ship::crd();
    let ship_yaml = serde_yaml::to_string(&ship_crd)?;
    print!("{}", ship_yaml);
    println!("---");

    Ok(())
}

pub(crate) async fn create_namespace(name: &str) -> Result<()> {
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
