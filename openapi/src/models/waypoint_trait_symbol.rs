/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// WaypointTraitSymbol : The unique identifier of the trait.

/// The unique identifier of the trait.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WaypointTraitSymbol {
    #[serde(rename = "UNCHARTED")]
    Uncharted,
    #[serde(rename = "UNDER_CONSTRUCTION")]
    UnderConstruction,
    #[serde(rename = "MARKETPLACE")]
    Marketplace,
    #[serde(rename = "SHIPYARD")]
    Shipyard,
    #[serde(rename = "OUTPOST")]
    Outpost,
    #[serde(rename = "SCATTERED_SETTLEMENTS")]
    ScatteredSettlements,
    #[serde(rename = "SPRAWLING_CITIES")]
    SprawlingCities,
    #[serde(rename = "MEGA_STRUCTURES")]
    MegaStructures,
    #[serde(rename = "PIRATE_BASE")]
    PirateBase,
    #[serde(rename = "OVERCROWDED")]
    Overcrowded,
    #[serde(rename = "HIGH_TECH")]
    HighTech,
    #[serde(rename = "CORRUPT")]
    Corrupt,
    #[serde(rename = "BUREAUCRATIC")]
    Bureaucratic,
    #[serde(rename = "TRADING_HUB")]
    TradingHub,
    #[serde(rename = "INDUSTRIAL")]
    Industrial,
    #[serde(rename = "BLACK_MARKET")]
    BlackMarket,
    #[serde(rename = "RESEARCH_FACILITY")]
    ResearchFacility,
    #[serde(rename = "MILITARY_BASE")]
    MilitaryBase,
    #[serde(rename = "SURVEILLANCE_OUTPOST")]
    SurveillanceOutpost,
    #[serde(rename = "EXPLORATION_OUTPOST")]
    ExplorationOutpost,
    #[serde(rename = "MINERAL_DEPOSITS")]
    MineralDeposits,
    #[serde(rename = "COMMON_METAL_DEPOSITS")]
    CommonMetalDeposits,
    #[serde(rename = "PRECIOUS_METAL_DEPOSITS")]
    PreciousMetalDeposits,
    #[serde(rename = "RARE_METAL_DEPOSITS")]
    RareMetalDeposits,
    #[serde(rename = "METHANE_POOLS")]
    MethanePools,
    #[serde(rename = "ICE_CRYSTALS")]
    IceCrystals,
    #[serde(rename = "EXPLOSIVE_GASES")]
    ExplosiveGases,
    #[serde(rename = "STRONG_MAGNETOSPHERE")]
    StrongMagnetosphere,
    #[serde(rename = "VIBRANT_AURORAS")]
    VibrantAuroras,
    #[serde(rename = "SALT_FLATS")]
    SaltFlats,
    #[serde(rename = "CANYONS")]
    Canyons,
    #[serde(rename = "PERPETUAL_DAYLIGHT")]
    PerpetualDaylight,
    #[serde(rename = "PERPETUAL_OVERCAST")]
    PerpetualOvercast,
    #[serde(rename = "DRY_SEABEDS")]
    DrySeabeds,
    #[serde(rename = "MAGMA_SEAS")]
    MagmaSeas,
    #[serde(rename = "SUPERVOLCANOES")]
    Supervolcanoes,
    #[serde(rename = "ASH_CLOUDS")]
    AshClouds,
    #[serde(rename = "VAST_RUINS")]
    VastRuins,
    #[serde(rename = "MUTATED_FLORA")]
    MutatedFlora,
    #[serde(rename = "TERRAFORMED")]
    Terraformed,
    #[serde(rename = "EXTREME_TEMPERATURES")]
    ExtremeTemperatures,
    #[serde(rename = "EXTREME_PRESSURE")]
    ExtremePressure,
    #[serde(rename = "DIVERSE_LIFE")]
    DiverseLife,
    #[serde(rename = "SCARCE_LIFE")]
    ScarceLife,
    #[serde(rename = "FOSSILS")]
    Fossils,
    #[serde(rename = "WEAK_GRAVITY")]
    WeakGravity,
    #[serde(rename = "STRONG_GRAVITY")]
    StrongGravity,
    #[serde(rename = "CRUSHING_GRAVITY")]
    CrushingGravity,
    #[serde(rename = "TOXIC_ATMOSPHERE")]
    ToxicAtmosphere,
    #[serde(rename = "CORROSIVE_ATMOSPHERE")]
    CorrosiveAtmosphere,
    #[serde(rename = "BREATHABLE_ATMOSPHERE")]
    BreathableAtmosphere,
    #[serde(rename = "THIN_ATMOSPHERE")]
    ThinAtmosphere,
    #[serde(rename = "JOVIAN")]
    Jovian,
    #[serde(rename = "ROCKY")]
    Rocky,
    #[serde(rename = "VOLCANIC")]
    Volcanic,
    #[serde(rename = "FROZEN")]
    Frozen,
    #[serde(rename = "SWAMP")]
    Swamp,
    #[serde(rename = "BARREN")]
    Barren,
    #[serde(rename = "TEMPERATE")]
    Temperate,
    #[serde(rename = "JUNGLE")]
    Jungle,
    #[serde(rename = "OCEAN")]
    Ocean,
    #[serde(rename = "RADIOACTIVE")]
    Radioactive,
    #[serde(rename = "MICRO_GRAVITY_ANOMALIES")]
    MicroGravityAnomalies,
    #[serde(rename = "DEBRIS_CLUSTER")]
    DebrisCluster,
    #[serde(rename = "DEEP_CRATERS")]
    DeepCraters,
    #[serde(rename = "SHALLOW_CRATERS")]
    ShallowCraters,
    #[serde(rename = "UNSTABLE_COMPOSITION")]
    UnstableComposition,
    #[serde(rename = "HOLLOWED_INTERIOR")]
    HollowedInterior,
    #[serde(rename = "STRIPPED")]
    Stripped,
}

impl ToString for WaypointTraitSymbol {
    fn to_string(&self) -> String {
        match self {
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
        }
    }
}

impl Default for WaypointTraitSymbol {
    fn default() -> WaypointTraitSymbol {
        Self::Uncharted
    }
}
