use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Binary;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub drand_public_key: Binary,
}

pub const CONFIG: Item<Config> = Item::new("config");
