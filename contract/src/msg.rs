use cosmwasm_std::Empty;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// We don't take any parameter for instantiation
pub type InstantiateMsg = Empty;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Query the sequence number of an account. Returns `u64`
    Sequence {
        address: String,
    },
}
