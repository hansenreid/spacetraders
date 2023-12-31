/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// MarketTransaction : Result of a transaction with a market.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketTransaction {
    /// The symbol of the waypoint.
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    /// The symbol of the ship that made the transaction.
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    /// The symbol of the trade good.
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    /// The type of transaction.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The number of units of the transaction.
    #[serde(rename = "units")]
    pub units: i32,
    /// The price per unit of the transaction.
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: i32,
    /// The total price of the transaction.
    #[serde(rename = "totalPrice")]
    pub total_price: i32,
    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

impl MarketTransaction {
    /// Result of a transaction with a market.
    pub fn new(
        waypoint_symbol: String,
        ship_symbol: String,
        trade_symbol: String,
        r#type: Type,
        units: i32,
        price_per_unit: i32,
        total_price: i32,
        timestamp: String,
    ) -> MarketTransaction {
        MarketTransaction {
            waypoint_symbol,
            ship_symbol,
            trade_symbol,
            r#type,
            units,
            price_per_unit,
            total_price,
            timestamp,
        }
    }
}

/// The type of transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "PURCHASE")]
    Purchase,
    #[serde(rename = "SELL")]
    Sell,
}

impl Default for Type {
    fn default() -> Type {
        Self::Purchase
    }
}