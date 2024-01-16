use common::crds::Manager;
use eyre::{Context, Ok, Result};
use futures::StreamExt;
use kube::runtime::Controller;
use kube::CustomResourceExt;
use kube::{Api, Client, Config};
use snafu::Snafu;
use std::sync::Arc;
use tracing::info;

mod agent;
mod ship;

mod reconcile;
pub use reconcile::init_manager;

#[derive(Debug, Snafu)]
pub enum Error {}

async fn get_client() -> Result<Client> {
    let config = Config::infer().await?;
    assert!(config.cluster_url.to_string() == "https://0.0.0.0:6444/");
    Client::try_from(config).wrap_err("error creating k8s client")
}

pub async fn run() -> Result<()> {
    info!("Running operator");
    let client = get_client().await?;
    let manager = Api::<Manager>::all(client);

    Controller::new(manager.clone(), Default::default())
        .run(reconcile::reconcile, reconcile::error_policy, Arc::new(()))
        .for_each(|_| futures::future::ready(()))
        .await;

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
