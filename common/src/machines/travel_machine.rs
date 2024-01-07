use std::time::Duration;

use eyre::{Context, Ok, Result};
use openapi::apis::configuration::Configuration;
use openapi::apis::fleet_api;
use sea_orm::DatabaseConnection;

use crate::models::ship::{ShipNav, ShipNavStatus};
use crate::models::{Location, Ship};

pub enum TravelMachineWrapper<'a> {
    Docked(TravelMachine<'a, Docked>),
    InOrbit(TravelMachine<'a, InOrbit>),
    InTransit(TravelMachine<'a, InTransit>),
    Arrived(TravelMachine<'a, Arrived>),
    TravelComplete,
}

impl<'a> TravelMachineWrapper<'a> {
    pub async fn new(
        config: Configuration,
        db: &'a DatabaseConnection,
        destination: Location,
        ship_symbol: &str,
    ) -> Result<Self> {
        let res = openapi::apis::fleet_api::get_my_ship(&config, ship_symbol).await?;
        let ship = Ship::from(res.data);

        match ship.nav.status {
            crate::models::ship::ShipNavStatus::Docked => {
                if ship.nav.location == destination {
                    Ok(Self::TravelComplete)
                } else {
                    Ok(Self::Docked(TravelMachine::<Docked>::new(
                        config,
                        db,
                        destination,
                        ship,
                    )))
                }
            }

            crate::models::ship::ShipNavStatus::InOrbit => {
                if ship.nav.location == destination {
                    Ok(Self::Arrived(TravelMachine::<Arrived>::new(
                        config,
                        db,
                        destination,
                        ship,
                    )))
                } else {
                    Ok(Self::InOrbit(TravelMachine::<InOrbit>::new(
                        config,
                        db,
                        destination,
                        ship,
                    )))
                }
            }

            crate::models::ship::ShipNavStatus::InTransit => {
                Ok(Self::InTransit(TravelMachine::<InTransit>::new(
                    config,
                    db,
                    destination,
                    ship,
                )))
            }
        }
    }

    pub async fn step(self) -> Result<Self> {
        tokio::time::sleep(Duration::from_millis(5000)).await;
        match self {
            TravelMachineWrapper::Docked(val) => {
                if val.destination == val.ship.nav.location {
                    Ok(TravelMachineWrapper::TravelComplete)
                } else {
                    Ok(TravelMachineWrapper::InOrbit(val.undock().await?))
                }
            }

            TravelMachineWrapper::InOrbit(val) => {
                Ok(TravelMachineWrapper::InTransit(val.travel().await?))
            }

            TravelMachineWrapper::InTransit(val) => {
                if let Some(t) = val.ship.nav.route.time_to_arrival {
                    println!("Ship is in transit. The ship will arrive in {}", t);
                    Ok(TravelMachineWrapper::InTransit(val))
                } else {
                    println!("Ship is in transit. Arrival time not available");
                    Ok(TravelMachineWrapper::InTransit(val))
                }
            }
            TravelMachineWrapper::Arrived(val) => {
                Ok(TravelMachineWrapper::Docked(val.dock().await?))
            }
            TravelMachineWrapper::TravelComplete => {
                Err(eyre::eyre!("Travel has already been completed"))
            }
        }
    }
}

pub struct TravelMachine<'a, S> {
    pub state: S,
    db: &'a DatabaseConnection,
    config: Configuration,
    destination: Location,
    ship: Ship,
}

pub struct Docked;
impl<'a> TravelMachine<'a, Docked> {
    pub fn new(
        config: Configuration,
        db: &'a DatabaseConnection,
        destination: Location,
        ship: Ship,
    ) -> Self {
        Self {
            config,
            db,
            destination,
            ship,
            state: Docked,
        }
    }

    pub async fn undock(mut self) -> Result<TravelMachine<'a, InOrbit>> {
        let res = fleet_api::orbit_ship(&self.config, self.ship.symbol.as_str())
            .await
            .wrap_err("Error undocking")?;

        let ship_nav = ShipNav::from(res.data.nav);
        if ship_nav.status != ShipNavStatus::InOrbit {
            return Err(eyre::eyre!("Failed to enter orbit!"));
        }

        self.ship.update_nav(ship_nav);
        println!("Ship undocked");

        Ok(TravelMachine::<InOrbit>::new(
            self.config,
            self.db,
            self.destination,
            self.ship,
        ))
    }
}

pub struct InOrbit;
impl<'a> TravelMachine<'a, InOrbit> {
    pub fn new(
        config: Configuration,
        db: &'a DatabaseConnection,
        destination: Location,
        ship: Ship,
    ) -> Self {
        Self {
            config,
            db,
            destination,
            ship,
            state: InOrbit,
        }
    }

    pub async fn travel(mut self) -> Result<TravelMachine<'a, InTransit>> {
        let nav = openapi::models::navigate_ship_request::NavigateShipRequest {
            waypoint_symbol: self.destination.to_string(),
        };

        let waypoints = crate::repository::get_marketplace_waypoints(self.db).await?;
        println!("Waypoints Found: {}", waypoints.len());

        let res =
            fleet_api::navigate_ship(&self.config, self.ship.symbol.as_str(), Some(nav)).await;

        match res {
            std::result::Result::Ok(res) => {
                let ship_nav = ShipNav::from(res.data.nav);
                if ship_nav.status != ShipNavStatus::InTransit {
                    return Err(eyre::eyre!("Failed launch ship!"));
                }

                self.ship.update_nav(ship_nav);
                println!("Ship launched");

                Ok(TravelMachine::<InTransit>::new(
                    self.config,
                    self.db,
                    self.destination,
                    self.ship,
                ))
            }
            Err(e) => {
                println!("ERROR: {:#?}", e);
                Err(e).wrap_err("Error launching ship")
            }
        }
    }
}

pub struct InTransit;
impl<'a> TravelMachine<'a, InTransit> {
    pub fn new(
        config: Configuration,
        db: &'a DatabaseConnection,
        destination: Location,
        ship: Ship,
    ) -> Self {
        Self {
            config,
            db,
            destination,
            ship,
            state: InTransit,
        }
    }
}

pub struct Arrived;
impl<'a> TravelMachine<'a, Arrived> {
    pub fn new(
        config: Configuration,
        db: &'a DatabaseConnection,
        destination: Location,
        ship: Ship,
    ) -> Self {
        Self {
            config,
            db,
            destination,
            ship,
            state: Arrived,
        }
    }

    pub async fn dock(mut self) -> Result<TravelMachine<'a, Docked>> {
        let res = fleet_api::dock_ship(&self.config, self.ship.symbol.as_str()).await?;
        let ship_nav = ShipNav::from(res.data.nav);
        if ship_nav.status != ShipNavStatus::Docked {
            return Err(eyre::eyre!("Failed to dock ship!"));
        }

        self.ship.update_nav(ship_nav);
        println!("Ship docked");

        Ok(TravelMachine::<Docked>::new(
            self.config,
            self.db,
            self.destination,
            self.ship,
        ))
    }
}
