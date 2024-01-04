use eyre::{Context, Ok, Result};
use openapi::apis::configuration::Configuration;
use openapi::apis::fleet_api;

use crate::models::ship::{ShipNav, ShipNavStatus};
use crate::models::{Location, Ship};

pub enum TravelMachineWrapper {
    Docked(TravelMachine<Docked>),
    InOrbit(TravelMachine<InOrbit>),
    InTransit(TravelMachine<InTransit>),
    Arrived(TravelMachine<Arrived>),
}

impl TravelMachineWrapper {
    pub async fn new(
        config: Configuration,
        destination: Location,
        ship_symbol: &str,
    ) -> Result<Self> {
        let res = openapi::apis::fleet_api::get_my_ship(&config, ship_symbol).await?;
        let ship = Ship::from(res.data);

        if ship.nav.location == destination && ship.nav.status == ShipNavStatus::Docked {
            return Ok(Self::Arrived(TravelMachine::<Arrived>::new(
                config,
                destination,
                ship,
            )));
        }

        match ship.nav.status {
            crate::models::ship::ShipNavStatus::Docked => Ok(Self::Docked(
                TravelMachine::<Docked>::new(config, destination, ship),
            )),

            crate::models::ship::ShipNavStatus::InOrbit => {
                Ok(Self::InOrbit(TravelMachine::<InOrbit>::new(
                    config,
                    destination,
                    ship,
                )))
            }

            crate::models::ship::ShipNavStatus::InTransit => {
                Ok(Self::InTransit(TravelMachine::<InTransit>::new(
                    config,
                    destination,
                    ship,
                )))
            }
        }
    }

    pub async fn step(self) -> Result<Self> {
        let next = match self {
            TravelMachineWrapper::Docked(val) => TravelMachineWrapper::InOrbit(val.undock().await?),
            TravelMachineWrapper::InOrbit(val) => TravelMachineWrapper::Docked(val.dock().await?),
            TravelMachineWrapper::InTransit(_) => todo!(),
            TravelMachineWrapper::Arrived(_) => todo!(),
        };

        Ok(next)
    }
}

pub struct TravelMachine<S> {
    pub state: S,
    config: Configuration,
    destination: Location,
    ship: Ship,
}

pub struct Docked;
impl TravelMachine<Docked> {
    pub fn new(config: Configuration, destination: Location, ship: Ship) -> Self {
        Self {
            config,
            destination,
            ship,
            state: Docked,
        }
    }

    pub async fn undock(mut self) -> Result<TravelMachine<InOrbit>> {
        let res = fleet_api::orbit_ship(&self.config, self.ship.symbol.as_str())
            .await
            .wrap_err("Error undocking")?;

        let ship_nav = ShipNav::from(res.data.nav);
        if ship_nav.status != ShipNavStatus::InOrbit {
            return Err(eyre::eyre!("Failed to enter orbit!"));
        }

        self.ship.update_nav(ship_nav);

        Ok(TravelMachine::<InOrbit>::new(
            self.config,
            self.destination,
            self.ship,
        ))
    }
}

pub struct InOrbit;
impl TravelMachine<InOrbit> {
    pub fn new(config: Configuration, destination: Location, ship: Ship) -> Self {
        Self {
            config,
            destination,
            ship,
            state: InOrbit,
        }
    }

    pub async fn dock(mut self) -> Result<TravelMachine<Docked>> {
        let res = fleet_api::dock_ship(&self.config, self.ship.symbol.as_str()).await?;
        let ship_nav = ShipNav::from(res.data.nav);
        if ship_nav.status != ShipNavStatus::Docked {
            return Err(eyre::eyre!("Failed to dock ship!"));
        }

        self.ship.update_nav(ship_nav);
        Ok(TravelMachine::<Docked>::new(
            self.config,
            self.destination,
            self.ship,
        ))
    }
}

pub struct InTransit;
impl TravelMachine<InTransit> {
    pub fn new(config: Configuration, destination: Location, ship: Ship) -> Self {
        Self {
            config,
            destination,
            ship,
            state: InTransit,
        }
    }
}

pub struct Arrived;
impl TravelMachine<Arrived> {
    pub fn new(config: Configuration, destination: Location, ship: Ship) -> Self {
        Self {
            config,
            destination,
            ship,
            state: Arrived,
        }
    }
}
