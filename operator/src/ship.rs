use std::sync::Arc;
use std::time::Duration;

use common::models::ship::Ship;
use eyre::Result;
use futures::StreamExt;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference;
use kube::api::{Patch, PatchParams};
use kube::core::ObjectMeta;
use kube::runtime::controller::Action;
use kube::runtime::Controller;
use kube::{Api, Client, ResourceExt};

use common::crds::{Ship as K8sShip, ShipSpec, ShipStatus};
use openapi::apis;
use openapi::apis::fleet_api::{self, GetMyShipError};
use serde_json::json;
use snafu::{ensure, ResultExt, Snafu};
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

    #[snafu(display("error received from fleet api {}", source))]
    GetShipError { source: apis::Error<GetMyShipError> },

    #[snafu(display("error patching ship {}", source))]
    PatchShipError { source: kube::Error },
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
    k8s_ship: Arc<K8sShip>,
    ctx: Arc<ShipControllerData>,
) -> Result<Action, ShipError> {
    info!("Reconciling ship {}", k8s_ship.name_any());

    let cfg = ctx.api_config.read().await;
    ensure!(cfg.is_some(), ApiConfigNotAvailableSnafu);
    let cfg = cfg.as_ref().unwrap();

    let res = fleet_api::get_my_ship(cfg, k8s_ship.spec.symbol.as_str())
        .await
        .context(GetShipSnafu)?;

    let ship = Ship::from(res.data);
    let ns = k8s_ship.namespace().unwrap_or("default".to_string());
    let ship_api: Api<K8sShip> = Api::namespaced(ctx.k8s_client.clone(), ns.as_str());

    let status = json!({
        "status": ShipStatus {
            location: Some(ship.nav.location),
            status: Some(ship.nav.status),
            flight_mode: Some(ship.nav.flight_mode) }
    });

    let serverside = PatchParams::apply("operator");
    ship_api
        .patch_status(
            k8s_ship.name_any().as_str(),
            &serverside,
            &Patch::Merge(&status),
        )
        .await
        .context(PatchShipSnafu)?;

    info!("ship {} reconciled", k8s_ship.name_any());
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
            location: None,
            status: None,
            flight_mode: None,
        }),
    }
}
