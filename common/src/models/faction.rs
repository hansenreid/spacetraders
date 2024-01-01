use openapi::models::FactionSymbol as ApiFaction;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use tabled::Tabled;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub enum FactionSymbol {
    Cosmic,
    Void,
    Galactic,
    Quantum,
    Dominion,
    Astro,
    Corsairs,
    Obsidian,
    Aegis,
    United,
    Solitary,
    Cobalt,
    Omega,
    Echo,
    Lords,
    Cult,
    Ancients,
    Shadow,
    Ethereal,
}

impl FactionSymbol {
    pub fn parse(input: String) -> Self {
        match Self::from_str(input.as_str()) {
            Ok(symbol) => symbol,
            Err(e) => {
                println!("Failed to parse faction {}: {:?}", input, e);
                panic!()
            }
        }
    }

    pub fn to_vec() -> Vec<&'static str> {
        vec![
            "COSMIC", "VOID", "GALACTIC", "QUANTUM", "DOMINION", "ASTRO", "CORSAIRS", "OBSIDIAN",
            "AEGIS", "UNITED", "SOLITARY", "COBALT", "OMEGA", "ECHO", "LORDS", "CULT", "ANCIENTS",
            "SHADOW", "ETHEREAL",
        ]
    }
}

impl Display for FactionSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Cosmic => String::from("COSMIC"),
            Self::Void => String::from("VOID"),
            Self::Galactic => String::from("GALACTIC"),
            Self::Quantum => String::from("QUANTUM"),
            Self::Dominion => String::from("DOMINION"),
            Self::Astro => String::from("ASTRO"),
            Self::Corsairs => String::from("CORSAIRS"),
            Self::Obsidian => String::from("OBSIDIAN"),
            Self::Aegis => String::from("AEGIS"),
            Self::United => String::from("UNITED"),
            Self::Solitary => String::from("SOLITARY"),
            Self::Cobalt => String::from("COBALT"),
            Self::Omega => String::from("OMEGA"),
            Self::Echo => String::from("ECHO"),
            Self::Lords => String::from("LORDS"),
            Self::Cult => String::from("CULT"),
            Self::Ancients => String::from("ANCIENTS"),
            Self::Shadow => String::from("SHADOW"),
            Self::Ethereal => String::from("ETHEREAL"),
        };

        write!(f, "{}", string)
    }
}

impl Into<openapi::models::FactionSymbol> for FactionSymbol {
    fn into(self) -> openapi::models::FactionSymbol {
        match self {
            Self::Cosmic => ApiFaction::Cosmic,
            Self::Void => ApiFaction::Void,
            Self::Galactic => ApiFaction::Galactic,
            Self::Quantum => ApiFaction::Quantum,
            Self::Dominion => ApiFaction::Dominion,
            Self::Astro => ApiFaction::Astro,
            Self::Corsairs => ApiFaction::Corsairs,
            Self::Obsidian => ApiFaction::Obsidian,
            Self::Aegis => ApiFaction::Aegis,
            Self::United => ApiFaction::United,
            Self::Solitary => ApiFaction::Solitary,
            Self::Cobalt => ApiFaction::Cobalt,
            Self::Omega => ApiFaction::Omega,
            Self::Echo => ApiFaction::Echo,
            Self::Lords => ApiFaction::Lords,
            Self::Cult => ApiFaction::Cult,
            Self::Ancients => ApiFaction::Ancients,
            Self::Shadow => ApiFaction::Shadow,
            Self::Ethereal => ApiFaction::Ethereal,
        }
    }
}

impl FromStr for FactionSymbol {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "COSMIC" => Ok(Self::Cosmic),
            "VOID" => Ok(Self::Void),
            "GALACTIC" => Ok(Self::Galactic),
            "QUANTUM" => Ok(Self::Quantum),
            "DOMINION" => Ok(Self::Dominion),
            "ASTRO" => Ok(Self::Astro),
            "CORSAIRS" => Ok(Self::Corsairs),
            "OBSIDIAN" => Ok(Self::Obsidian),
            "AEGIS" => Ok(Self::Aegis),
            "UNITED" => Ok(Self::United),
            "SOLITARY" => Ok(Self::Solitary),
            "COBALT" => Ok(Self::Cobalt),
            "OMEGA" => Ok(Self::Omega),
            "ECHO" => Ok(Self::Echo),
            "LORDS" => Ok(Self::Lords),
            "CULT" => Ok(Self::Cult),
            "ANCIENTS" => Ok(Self::Ancients),
            "SHADOW" => Ok(Self::Shadow),
            "ETHEREAL" => Ok(Self::Ethereal),
            _ => Err(eyre::eyre!("Unkown faction")),
        }
    }
}
