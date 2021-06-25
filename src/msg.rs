use crate::state::Config;
use cosmwasm_std::Binary;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Get the config state
    Config {},
    /// Not used to be call directly
    VerifyCallBack {
        round: u64,
        randomness: Binary,
        valid: bool,
        worker: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Verify {
        signature: Binary,
        msg_g2: Binary,
        worker: String,
        round: u64,
    },
}
// We define a custom struct for each query response
pub type ConfigResponse = Config;
