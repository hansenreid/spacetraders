use eyre::Result;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference;
use kube::api::{Patch, PatchParams};
use kube::core::ObjectMeta;
use kube::{Api, Client, ResourceExt};

use common::crds::{Ship as K8sShip, ShipSpec, ShipStatus};

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
