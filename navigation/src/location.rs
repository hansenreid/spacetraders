use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct Location {
    pub sector: String,
    pub system: String,
    pub waypoint: String,
}

impl Location {
    pub fn parse(input: String) -> Self {
        match Self::from_str(input.as_str()) {
            Ok(location) => location,
            Err(e) => {
                println!("Failed to parse location {}: {:?}", input, e);
                panic!()
            }
        }
    }

    pub fn system_ident(&self) -> String {
        format!("{}-{}", self.sector, self.system)
    }

    pub fn waypoint_ident(&self) -> String {
        self.to_string()
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.sector, self.system, self.waypoint)
    }
}

impl FromStr for Location {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split("-").collect::<Vec<&str>>().as_slice() {
            [sector, system, waypoint] => Ok(Self {
                sector: sector.to_string(),
                system: system.to_string(),
                waypoint: waypoint.to_string(),
            }),

            _ => Err(eyre::eyre!("Failed to parse location")),
        }
    }
}
