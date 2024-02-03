use std::sync::Arc;
use std::time::Duration;

use eyre::Result;
use futures::StreamExt;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference;
use kube::api::{Patch, PatchParams};
use kube::core::ObjectMeta;
use kube::runtime::controller::Action;
use kube::runtime::Controller;
use kube::{Api, Client, ResourceExt};

use common::crds::{Ship as K8sShip, ShipSpec, ShipStatus};
use snafu::{ensure, Snafu};
use tracing::{info, warn};

pub struct ShipControllerData {
    pub api_config: crate::ApiConfig,
    pub k8s_client: Client,
}

impl ShipControllerData {
    pub fn new(k8s_client: Client, api_config: crate::ApiConfig) -> Self {
        Self {
            api_config,
            k8s_client,
        }
    }
}

#[derive(Debug, Snafu)]
pub enum ShipError {
    #[snafu(display("api config is not available yet"))]
    ApiConfigNotAvailable,
}

pub(crate) async fn run_controller(data: Arc<ShipControllerData>) -> eyre::Result<()> {
    let ship = Api::<K8sShip>::all(data.k8s_client.clone());

    Controller::new(ship.clone(), Default::default())
        .run(reconcile, error_policy, data)
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(())
}

pub(crate) fn error_policy(
    _object: Arc<K8sShip>,
    err: &ShipError,
    _ctx: Arc<ShipControllerData>,
) -> Action {
    warn!("error reconcilling ships: {}", err);
    Action::requeue(Duration::from_secs(5))
}

pub(crate) async fn reconcile(
    ship: Arc<K8sShip>,
    ctx: Arc<ShipControllerData>,
) -> Result<Action, ShipError> {
    info!("Reconciling ships");

    let cfg = ctx.api_config.read().await;
    ensure!(cfg.is_some(), ApiConfigNotAvailableSnafu);

    info!("ship {} reconciled", ship.name_any());
    Ok(Action::await_change())
}

pub(crate) async fn patch_ship(
    client: Client,
    symbol: String,
    namespace: Option<String>,
    oref: Option<OwnerReference>,
) -> Result<()> {
    let ns = namespace.unwrap_or("default".to_string());
    let serverside = PatchParams::apply("operator");
    let ship_api: Api<K8sShip> = Api::namespaced(client, ns.as_str());

    let ship = create_owned_ship(symbol, ns, oref);

    ship_api
        .patch(ship.name_any().as_str(), &serverside, &Patch::Apply(&ship))
        .await?;

    ship_api
        .patch_status(ship.name_any().as_str(), &serverside, &Patch::Apply(&ship))
        .await?;

    Ok(())
}

fn create_owned_ship(symbol: String, namespace: String, oref: Option<OwnerReference>) -> K8sShip {
    let name = symbol.to_lowercase();
    let spec = ShipSpec { symbol };
    let owner_references = oref.map_or(None, |oref| Some(vec![oref]));

    K8sShip {
        metadata: ObjectMeta {
            name: Some(name),
            namespace: Some(namespace),
            owner_references,
            ..Default::default()
        },
        spec,
        status: Some(ShipStatus {
            checksum: "".into(),
            last_updated: None,
        }),
    }
}
