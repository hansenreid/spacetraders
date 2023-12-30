use crate::ApiConfig;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {}

pub async fn register(_config: ApiConfig) -> Result<Register> {
todo!()
}
