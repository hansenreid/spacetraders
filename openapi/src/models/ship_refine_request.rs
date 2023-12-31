/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipRefineRequest {
    /// The type of good to produce out of the refining process.
    #[serde(rename = "produce")]
    pub produce: Produce,
}

impl ShipRefineRequest {
    pub fn new(produce: Produce) -> ShipRefineRequest {
        ShipRefineRequest { produce }
    }
}

/// The type of good to produce out of the refining process.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Produce {
    #[serde(rename = "IRON")]
    Iron,
    #[serde(rename = "COPPER")]
    Copper,
    #[serde(rename = "SILVER")]
    Silver,
    #[serde(rename = "GOLD")]
    Gold,
    #[serde(rename = "ALUMINUM")]
    Aluminum,
    #[serde(rename = "PLATINUM")]
    Platinum,
    #[serde(rename = "URANITE")]
    Uranite,
    #[serde(rename = "MERITIUM")]
    Meritium,
    #[serde(rename = "FUEL")]
    Fuel,
}

impl Default for Produce {
    fn default() -> Produce {
        Self::Iron
    }
}
