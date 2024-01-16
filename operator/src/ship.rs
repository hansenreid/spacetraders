use eyre::Result;
use kube::api::{Patch, PatchParams};
use kube::core::ObjectMeta;
use kube::{Api, Resource, ResourceExt};
use std::sync::Arc;
use tracing::info;

use common::crds::{
    Agent as K8sAgent, AgentStatus, Manager, Ship as K8sShip, ShipSpec, ShipStatus,
};

use crate::agent::patch_status;

pub(crate) async fn reconcile_starting_ships(manager: Arc<Manager>) -> Result<()> {
    info!("reconciling starting ships");
    let agent = crate::agent::get_agent(manager.clone()).await?;
    if let Some(status) = &agent.status {
        if !status.ships_initialized {
            patch_k8s_ships(manager.clone(), &agent).await?;
            let updated_status = AgentStatus {
                checksum: "".into(),
                ships_initialized: true,
                last_updated: None,
            };

            patch_status(manager.clone(), &updated_status).await?;
        }
    }

    info!("starting ships reconciled");
    Ok(())
}

async fn patch_k8s_ships(manager: Arc<Manager>, agent: &K8sAgent) -> Result<()> {
    let ships = get_ships(&agent).await?;
    for ship in ships {
        patch_k8s_ship(manager.clone(), ship).await?;
    }

    Ok(())
}

async fn patch_k8s_ship(manager: Arc<Manager>, ship: common::models::Ship) -> Result<()> {
    let serverside = PatchParams::apply("operator");
    let client = crate::get_client().await.unwrap();
    let ship_api: Api<K8sShip> = Api::namespaced(client, manager.spec.namespace.clone().as_str());

    let ship = create_owned_ship(manager, ship);

    ship_api
        .patch(ship.name_any().as_str(), &serverside, &Patch::Apply(&ship))
        .await?;

    ship_api
        .patch_status(ship.name_any().as_str(), &serverside, &Patch::Apply(&ship))
        .await?;

    Ok(())
}

fn create_owned_ship(manager: Arc<Manager>, ship: common::models::Ship) -> K8sShip {
    let oref = manager.controller_owner_ref(&()).unwrap();
    let name = ship.symbol.to_lowercase();
    let spec = ShipSpec {
        symbol: ship.symbol,
        ship_type: ship.registration.role,
    };

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

async fn get_ships(agent: &K8sAgent) -> Result<Vec<common::models::Ship>> {
    if let Some(token) = agent.spec.token.clone() {
        let conf = openapi::apis::configuration::Configuration {
            bearer_access_token: Some(token),
            ..Default::default()
        };

        let page = 1;
        let limit = 10;
        let res = openapi::apis::fleet_api::get_my_ships(&conf, Some(page), Some(limit)).await?;

        let mut ships = res
            .data
            .into_iter()
            .map(|ship| common::models::Ship::from(ship))
            .collect::<Vec<common::models::Ship>>();

        let total = res.meta.total;
        let num_pages = (total as f32 / limit as f32).ceil() as i32;

        for n in (page + 1)..=num_pages {
            let res = openapi::apis::fleet_api::get_my_ships(&conf, Some(n), Some(limit)).await?;
            for s in res.data {
                let ship = common::models::Ship::from(s);

                ships.push(ship)
            }
        }

        return Ok(ships);
    }

    Err(eyre::eyre!("no token found for agent"))
}
