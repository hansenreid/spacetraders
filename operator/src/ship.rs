use eyre::Result;
use kube::api::{Patch, PatchParams};
use kube::core::ObjectMeta;
use kube::{Api, Resource, ResourceExt};
use std::sync::Arc;

use common::crds::{Manager, Ship as K8sShip, ShipSpec, ShipStatus};

pub(crate) async fn patch_ship(manager: Arc<Manager>, symbol: String) -> Result<()> {
    let serverside = PatchParams::apply("operator");
    let client = crate::get_client().await.unwrap();
    let ship_api: Api<K8sShip> = Api::namespaced(client, manager.spec.namespace.clone().as_str());

    let ship = create_owned_ship(manager, symbol);

    ship_api
        .patch(ship.name_any().as_str(), &serverside, &Patch::Apply(&ship))
        .await?;

    ship_api
        .patch_status(ship.name_any().as_str(), &serverside, &Patch::Apply(&ship))
        .await?;

    Ok(())
}

fn create_owned_ship(manager: Arc<Manager>, symbol: String) -> K8sShip {
    let oref = manager.controller_owner_ref(&()).unwrap();
    let name = symbol.to_lowercase();
    let spec = ShipSpec { symbol };

    K8sShip {
        metadata: ObjectMeta {
            name: Some(name),
            namespace: Some(manager.spec.namespace.clone()),
            owner_references: Some(vec![oref]),
            ..Default::default()
        },
        spec,
        status: Some(ShipStatus {
            checksum: "".into(),
            last_updated: None,
        }),
    }
}
