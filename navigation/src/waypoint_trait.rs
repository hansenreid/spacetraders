use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct WaypointTrait {
    pub symbol: WaypointTraitSymbol,
}

#[derive(Debug)]
pub enum WaypointTraitSymbol {
    Uncharted,
    UnderConstruction,
    Marketplace,
    Shipyard,
    Outpost,
    ScatteredSettlements,
    SprawlingCities,
    MegaStructures,
    PirateBase,
    Overcrowded,
    HighTech,
    Corrupt,
    Bureaucratic,
    TradingHub,
    Industrial,
    BlackMarket,
    ResearchFacility,
    MilitaryBase,
    SurveillanceOutpost,
    ExplorationOutpost,
    MineralDeposits,
    CommonMetalDeposits,
    PreciousMetalDeposits,
    RareMetalDeposits,
    MethanePools,
    IceCrystals,
    ExplosiveGases,
    StrongMagnetosphere,
    VibrantAuroras,
    SaltFlats,
    Canyons,
    PerpetualDaylight,
    PerpetualOvercast,
    DrySeabeds,
    MagmaSeas,
    Supervolcanoes,
    AshClouds,
    VastRuins,
    MutatedFlora,
    Terraformed,
    ExtremeTemperatures,
    ExtremePressure,
    DiverseLife,
    ScarceLife,
    Fossils,
    WeakGravity,
    StrongGravity,
    CrushingGravity,
    ToxicAtmosphere,
    CorrosiveAtmosphere,
    BreathableAtmosphere,
    ThinAtmosphere,
    Jovian,
    Rocky,
    Volcanic,
    Frozen,
    Swamp,
    Barren,
    Temperate,
    Jungle,
    Ocean,
    Radioactive,
    MicroGravityAnomalies,
    DebrisCluster,
    DeepCraters,
    ShallowCraters,
    UnstableComposition,
    HollowedInterior,
    Stripped,
}

impl WaypointTrait {
    pub fn parse_vec(v: String) -> Vec<WaypointTrait> {
        v.split(",")
            .filter_map(|s| match WaypointTraitSymbol::from_str(s) {
                Ok(t) => Some(Self { symbol: t }),
                Err(e) => {
                    println!("Failed to parse trait \"{}\": {}", s, e);
                    None
                }
            })
            .collect::<Vec<WaypointTrait>>()
    }
}

impl From<openapi::models::WaypointTrait> for WaypointTrait {
    fn from(value: openapi::models::WaypointTrait) -> Self {
        Self {
            symbol: value.symbol.into(),
        }
    }
}

impl Display for WaypointTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol.to_string())
    }
}

impl From<openapi::models::WaypointTraitSymbol> for WaypointTraitSymbol {
    fn from(value: openapi::models::WaypointTraitSymbol) -> Self {
        match value {
            openapi::models::WaypointTraitSymbol::Uncharted => Self::Uncharted,
            openapi::models::WaypointTraitSymbol::UnderConstruction => Self::UnderConstruction,
            openapi::models::WaypointTraitSymbol::Marketplace => Self::Marketplace,
            openapi::models::WaypointTraitSymbol::Shipyard => Self::Shipyard,
            openapi::models::WaypointTraitSymbol::Outpost => Self::Outpost,
            openapi::models::WaypointTraitSymbol::ScatteredSettlements => {
                Self::ScatteredSettlements
            }
            openapi::models::WaypointTraitSymbol::SprawlingCities => Self::SprawlingCities,
            openapi::models::WaypointTraitSymbol::MegaStructures => Self::MegaStructures,
            openapi::models::WaypointTraitSymbol::PirateBase => Self::PirateBase,
            openapi::models::WaypointTraitSymbol::Overcrowded => Self::Overcrowded,
            openapi::models::WaypointTraitSymbol::HighTech => Self::HighTech,
            openapi::models::WaypointTraitSymbol::Corrupt => Self::Corrupt,
            openapi::models::WaypointTraitSymbol::Bureaucratic => Self::Bureaucratic,
            openapi::models::WaypointTraitSymbol::TradingHub => Self::TradingHub,
            openapi::models::WaypointTraitSymbol::Industrial => Self::Industrial,
            openapi::models::WaypointTraitSymbol::BlackMarket => Self::BlackMarket,
            openapi::models::WaypointTraitSymbol::ResearchFacility => Self::ResearchFacility,
            openapi::models::WaypointTraitSymbol::MilitaryBase => Self::MilitaryBase,
            openapi::models::WaypointTraitSymbol::SurveillanceOutpost => Self::SurveillanceOutpost,
            openapi::models::WaypointTraitSymbol::ExplorationOutpost => Self::ExplorationOutpost,
            openapi::models::WaypointTraitSymbol::MineralDeposits => Self::MineralDeposits,
            openapi::models::WaypointTraitSymbol::CommonMetalDeposits => Self::CommonMetalDeposits,
            openapi::models::WaypointTraitSymbol::PreciousMetalDeposits => {
                Self::PreciousMetalDeposits
            }
            openapi::models::WaypointTraitSymbol::RareMetalDeposits => Self::RareMetalDeposits,
            openapi::models::WaypointTraitSymbol::MethanePools => Self::MethanePools,
            openapi::models::WaypointTraitSymbol::IceCrystals => Self::IceCrystals,
            openapi::models::WaypointTraitSymbol::ExplosiveGases => Self::ExplosiveGases,
            openapi::models::WaypointTraitSymbol::StrongMagnetosphere => Self::StrongMagnetosphere,
            openapi::models::WaypointTraitSymbol::VibrantAuroras => Self::VibrantAuroras,
            openapi::models::WaypointTraitSymbol::SaltFlats => Self::SaltFlats,
            openapi::models::WaypointTraitSymbol::Canyons => Self::Canyons,
            openapi::models::WaypointTraitSymbol::PerpetualDaylight => Self::PerpetualDaylight,
            openapi::models::WaypointTraitSymbol::PerpetualOvercast => Self::PerpetualOvercast,
            openapi::models::WaypointTraitSymbol::DrySeabeds => Self::DrySeabeds,
            openapi::models::WaypointTraitSymbol::MagmaSeas => Self::MagmaSeas,
            openapi::models::WaypointTraitSymbol::Supervolcanoes => Self::Supervolcanoes,
            openapi::models::WaypointTraitSymbol::AshClouds => Self::AshClouds,
            openapi::models::WaypointTraitSymbol::VastRuins => Self::VastRuins,
            openapi::models::WaypointTraitSymbol::MutatedFlora => Self::MutatedFlora,
            openapi::models::WaypointTraitSymbol::Terraformed => Self::Terraformed,
            openapi::models::WaypointTraitSymbol::ExtremeTemperatures => Self::ExtremeTemperatures,
            openapi::models::WaypointTraitSymbol::ExtremePressure => Self::ExtremePressure,
            openapi::models::WaypointTraitSymbol::DiverseLife => Self::DiverseLife,
            openapi::models::WaypointTraitSymbol::ScarceLife => Self::ScarceLife,
            openapi::models::WaypointTraitSymbol::Fossils => Self::Fossils,
            openapi::models::WaypointTraitSymbol::WeakGravity => Self::WeakGravity,
            openapi::models::WaypointTraitSymbol::StrongGravity => Self::StrongGravity,
            openapi::models::WaypointTraitSymbol::CrushingGravity => Self::CrushingGravity,
            openapi::models::WaypointTraitSymbol::ToxicAtmosphere => Self::ToxicAtmosphere,
            openapi::models::WaypointTraitSymbol::CorrosiveAtmosphere => Self::CorrosiveAtmosphere,
            openapi::models::WaypointTraitSymbol::BreathableAtmosphere => {
                Self::BreathableAtmosphere
            }
            openapi::models::WaypointTraitSymbol::ThinAtmosphere => Self::ThinAtmosphere,
            openapi::models::WaypointTraitSymbol::Jovian => Self::Jovian,
            openapi::models::WaypointTraitSymbol::Rocky => Self::Rocky,
            openapi::models::WaypointTraitSymbol::Volcanic => Self::Volcanic,
            openapi::models::WaypointTraitSymbol::Frozen => Self::Frozen,
            openapi::models::WaypointTraitSymbol::Swamp => Self::Swamp,
            openapi::models::WaypointTraitSymbol::Barren => Self::Barren,
            openapi::models::WaypointTraitSymbol::Temperate => Self::Temperate,
            openapi::models::WaypointTraitSymbol::Jungle => Self::Jungle,
            openapi::models::WaypointTraitSymbol::Ocean => Self::Ocean,
            openapi::models::WaypointTraitSymbol::Radioactive => Self::Radioactive,
            openapi::models::WaypointTraitSymbol::MicroGravityAnomalies => {
                Self::MicroGravityAnomalies
            }
            openapi::models::WaypointTraitSymbol::DebrisCluster => Self::DebrisCluster,
            openapi::models::WaypointTraitSymbol::DeepCraters => Self::DeepCraters,
            openapi::models::WaypointTraitSymbol::ShallowCraters => Self::ShallowCraters,
            openapi::models::WaypointTraitSymbol::UnstableComposition => Self::UnstableComposition,
            openapi::models::WaypointTraitSymbol::HollowedInterior => Self::HollowedInterior,
            openapi::models::WaypointTraitSymbol::Stripped => Self::Stripped,
        }
    }
}

impl FromStr for WaypointTraitSymbol {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UNCHARTED" => Ok(Self::Uncharted),
            "UNDER_CONSTRUCTION" => Ok(Self::UnderConstruction),
            "MARKETPLACE" => Ok(Self::Marketplace),
            "SHIPYARD" => Ok(Self::Shipyard),
            "OUTPOST" => Ok(Self::Outpost),
            "SCATTERED_SETTLEMENTS" => Ok(Self::ScatteredSettlements),
            "SPRAWLING_CITIES" => Ok(Self::SprawlingCities),
            "MEGA_STRUCTURES" => Ok(Self::MegaStructures),
            "PIRATE_BASE" => Ok(Self::PirateBase),
            "OVERCROWDED" => Ok(Self::Overcrowded),
            "HIGH_TECH" => Ok(Self::HighTech),
            "CORRUPT" => Ok(Self::Corrupt),
            "BUREAUCRATIC" => Ok(Self::Bureaucratic),
            "TRADING_HUB" => Ok(Self::TradingHub),
            "INDUSTRIAL" => Ok(Self::Industrial),
            "BLACK_MARKET" => Ok(Self::BlackMarket),
            "RESEARCH_FACILITY" => Ok(Self::ResearchFacility),
            "MILITARY_BASE" => Ok(Self::MilitaryBase),
            "SURVEILLANCE_OUTPOST" => Ok(Self::SurveillanceOutpost),
            "EXPLORATION_OUTPOST" => Ok(Self::ExplorationOutpost),
            "MINERAL_DEPOSITS" => Ok(Self::MineralDeposits),
            "COMMON_METAL_DEPOSITS" => Ok(Self::CommonMetalDeposits),
            "PRECIOUS_METAL_DEPOSITS" => Ok(Self::PreciousMetalDeposits),
            "RARE_METAL_DEPOSITS" => Ok(Self::RareMetalDeposits),
            "METHANE_POOLS" => Ok(Self::MethanePools),
            "ICE_CRYSTALS" => Ok(Self::IceCrystals),
            "EXPLOSIVE_GASES" => Ok(Self::ExplosiveGases),
            "STRONG_MAGNETOSPHERE" => Ok(Self::StrongMagnetosphere),
            "VIBRANT_AURORAS" => Ok(Self::VibrantAuroras),
            "SALT_FLATS" => Ok(Self::SaltFlats),
            "CANYONS" => Ok(Self::Canyons),
            "PERPETUAL_DAYLIGHT" => Ok(Self::PerpetualDaylight),
            "PERPETUAL_OVERCAST" => Ok(Self::PerpetualOvercast),
            "DRY_SEABEDS" => Ok(Self::DrySeabeds),
            "MAGMA_SEAS" => Ok(Self::MagmaSeas),
            "SUPERVOLCANOES" => Ok(Self::Supervolcanoes),
            "ASH_CLOUDS" => Ok(Self::AshClouds),
            "VAST_RUINS" => Ok(Self::VastRuins),
            "MUTATED_FLORA" => Ok(Self::MutatedFlora),
            "TERRAFORMED" => Ok(Self::Terraformed),
            "EXTREME_TEMPERATURES" => Ok(Self::ExtremeTemperatures),
            "EXTREME_PRESSURE" => Ok(Self::ExtremePressure),
            "DIVERSE_LIFE" => Ok(Self::DiverseLife),
            "SCARCE_LIFE" => Ok(Self::ScarceLife),
            "FOSSILS" => Ok(Self::Fossils),
            "WEAK_GRAVITY" => Ok(Self::WeakGravity),
            "STRONG_GRAVITY" => Ok(Self::StrongGravity),
            "CRUSHING_GRAVITY" => Ok(Self::CrushingGravity),
            "TOXIC_ATMOSPHERE" => Ok(Self::ToxicAtmosphere),
            "CORROSIVE_ATMOSPHERE" => Ok(Self::CorrosiveAtmosphere),
            "BREATHABLE_ATMOSPHERE" => Ok(Self::BreathableAtmosphere),
            "THIN_ATMOSPHERE" => Ok(Self::ThinAtmosphere),
            "JOVIAN" => Ok(Self::Jovian),
            "ROCKY" => Ok(Self::Rocky),
            "VOLCANIC" => Ok(Self::Volcanic),
            "FROZEN" => Ok(Self::Frozen),
            "SWAMP" => Ok(Self::Swamp),
            "BARREN" => Ok(Self::Barren),
            "TEMPERATE" => Ok(Self::Temperate),
            "JUNGLE" => Ok(Self::Jungle),
            "OCEAN" => Ok(Self::Ocean),
            "RADIOACTIVE" => Ok(Self::Radioactive),
            "MICRO_GRAVITY_ANOMALIES" => Ok(Self::MicroGravityAnomalies),
            "DEBRIS_CLUSTER" => Ok(Self::DebrisCluster),
            "DEEP_CRATERS" => Ok(Self::DeepCraters),
            "SHALLOW_CRATERS" => Ok(Self::ShallowCraters),
            "UNSTABLE_COMPOSITION" => Ok(Self::UnstableComposition),
            "HOLLOWED_INTERIOR" => Ok(Self::HollowedInterior),
            "STRIPPED" => Ok(Self::Stripped),
            _ => Err(eyre::eyre!("Unkown waypoint trait")),
        }
    }
}

impl Display for WaypointTraitSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Uncharted => String::from("UNCHARTED"),
            Self::UnderConstruction => String::from("UNDER_CONSTRUCTION"),
            Self::Marketplace => String::from("MARKETPLACE"),
            Self::Shipyard => String::from("SHIPYARD"),
            Self::Outpost => String::from("OUTPOST"),
            Self::ScatteredSettlements => String::from("SCATTERED_SETTLEMENTS"),
            Self::SprawlingCities => String::from("SPRAWLING_CITIES"),
            Self::MegaStructures => String::from("MEGA_STRUCTURES"),
            Self::PirateBase => String::from("PIRATE_BASE"),
            Self::Overcrowded => String::from("OVERCROWDED"),
            Self::HighTech => String::from("HIGH_TECH"),
            Self::Corrupt => String::from("CORRUPT"),
            Self::Bureaucratic => String::from("BUREAUCRATIC"),
            Self::TradingHub => String::from("TRADING_HUB"),
            Self::Industrial => String::from("INDUSTRIAL"),
            Self::BlackMarket => String::from("BLACK_MARKET"),
            Self::ResearchFacility => String::from("RESEARCH_FACILITY"),
            Self::MilitaryBase => String::from("MILITARY_BASE"),
            Self::SurveillanceOutpost => String::from("SURVEILLANCE_OUTPOST"),
            Self::ExplorationOutpost => String::from("EXPLORATION_OUTPOST"),
            Self::MineralDeposits => String::from("MINERAL_DEPOSITS"),
            Self::CommonMetalDeposits => String::from("COMMON_METAL_DEPOSITS"),
            Self::PreciousMetalDeposits => String::from("PRECIOUS_METAL_DEPOSITS"),
            Self::RareMetalDeposits => String::from("RARE_METAL_DEPOSITS"),
            Self::MethanePools => String::from("METHANE_POOLS"),
            Self::IceCrystals => String::from("ICE_CRYSTALS"),
            Self::ExplosiveGases => String::from("EXPLOSIVE_GASES"),
            Self::StrongMagnetosphere => String::from("STRONG_MAGNETOSPHERE"),
            Self::VibrantAuroras => String::from("VIBRANT_AURORAS"),
            Self::SaltFlats => String::from("SALT_FLATS"),
            Self::Canyons => String::from("CANYONS"),
            Self::PerpetualDaylight => String::from("PERPETUAL_DAYLIGHT"),
            Self::PerpetualOvercast => String::from("PERPETUAL_OVERCAST"),
            Self::DrySeabeds => String::from("DRY_SEABEDS"),
            Self::MagmaSeas => String::from("MAGMA_SEAS"),
            Self::Supervolcanoes => String::from("SUPERVOLCANOES"),
            Self::AshClouds => String::from("ASH_CLOUDS"),
            Self::VastRuins => String::from("VAST_RUINS"),
            Self::MutatedFlora => String::from("MUTATED_FLORA"),
            Self::Terraformed => String::from("TERRAFORMED"),
            Self::ExtremeTemperatures => String::from("EXTREME_TEMPERATURES"),
            Self::ExtremePressure => String::from("EXTREME_PRESSURE"),
            Self::DiverseLife => String::from("DIVERSE_LIFE"),
            Self::ScarceLife => String::from("SCARCE_LIFE"),
            Self::Fossils => String::from("FOSSILS"),
            Self::WeakGravity => String::from("WEAK_GRAVITY"),
            Self::StrongGravity => String::from("STRONG_GRAVITY"),
            Self::CrushingGravity => String::from("CRUSHING_GRAVITY"),
            Self::ToxicAtmosphere => String::from("TOXIC_ATMOSPHERE"),
            Self::CorrosiveAtmosphere => String::from("CORROSIVE_ATMOSPHERE"),
            Self::BreathableAtmosphere => String::from("BREATHABLE_ATMOSPHERE"),
            Self::ThinAtmosphere => String::from("THIN_ATMOSPHERE"),
            Self::Jovian => String::from("JOVIAN"),
            Self::Rocky => String::from("ROCKY"),
            Self::Volcanic => String::from("VOLCANIC"),
            Self::Frozen => String::from("FROZEN"),
            Self::Swamp => String::from("SWAMP"),
            Self::Barren => String::from("BARREN"),
            Self::Temperate => String::from("TEMPERATE"),
            Self::Jungle => String::from("JUNGLE"),
            Self::Ocean => String::from("OCEAN"),
            Self::Radioactive => String::from("RADIOACTIVE"),
            Self::MicroGravityAnomalies => String::from("MICRO_GRAVITY_ANOMALIES"),
            Self::DebrisCluster => String::from("DEBRIS_CLUSTER"),
            Self::DeepCraters => String::from("DEEP_CRATERS"),
            Self::ShallowCraters => String::from("SHALLOW_CRATERS"),
            Self::UnstableComposition => String::from("UNSTABLE_COMPOSITION"),
            Self::HollowedInterior => String::from("HOLLOWED_INTERIOR"),
            Self::Stripped => String::from("STRIPPED"),
        };

        write!(f, "{}", string)
    }
}
