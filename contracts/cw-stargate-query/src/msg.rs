use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Query information of a validator
    Validator(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ValidatorResponse {
    pub operator_address: String,
    pub jailed: bool,
    pub moniker: String,
    pub identity: String,
    pub commission_rate: Decimal,
}
