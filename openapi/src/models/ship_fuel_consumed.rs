/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// ShipFuelConsumed : An object that only shows up when an action has consumed fuel in the process. Shows the fuel consumption data.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipFuelConsumed {
    /// The amount of fuel consumed by the most recent transit or action.
    #[serde(rename = "amount")]
    pub amount: i32,
    /// The time at which the fuel was consumed.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

impl ShipFuelConsumed {
    /// An object that only shows up when an action has consumed fuel in the process. Shows the fuel consumption data.
    pub fn new(amount: i32, timestamp: String) -> ShipFuelConsumed {
        ShipFuelConsumed { amount, timestamp }
    }
}
