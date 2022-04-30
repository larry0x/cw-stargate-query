use std::io::Cursor;
use std::str::FromStr;

use cosmwasm_std::{
    entry_point, to_binary, Binary, Decimal, Deps, DepsMut, Empty, Env, MessageInfo,
    QuerierWrapper, QueryRequest, Response, StdError, StdResult,
};
use prost::Message;

use crate::msg::{QueryMsg, ValidatorResponse};

pub mod staking {
    include!(concat!(env!("OUT_DIR"), "/cosmos.staking.v1beta1.rs"));
}

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Err(StdError::generic_err("[cw-stargate-query]: execute is unimplemented"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Validator(validator_addr) => {
            to_binary(&query_validator(&deps.querier, validator_addr)?)
        }
    }
}

fn query_validator(
    querier: &QuerierWrapper,
    validator_addr: String,
) -> StdResult<ValidatorResponse> {
    let query = staking::QueryValidatorRequest {
        validator_addr,
    };
    let query_bin = Binary::from(query.encode_to_vec());

    let res_bin: Binary = querier.query(&QueryRequest::Stargate {
        path: "/cosmos.staking.v1beta1.Query/Validator".to_string(),
        data: query_bin,
    })?;
    let response = staking::QueryValidatorResponse::decode(&mut Cursor::new(res_bin.to_vec()))
        .map_err(|_| StdError::generic_err("failed to deserialize query response"))?;

    let validator = response.validator
        .ok_or_else(|| StdError::generic_err("failed to find the validator"))?;
    let description = validator
        .description
        .ok_or_else(|| StdError::generic_err("the validator's' description is undefined"))?;
    let commission = validator
        .commission
        .ok_or_else(|| StdError::generic_err("the validator's commission is undefined"))?;
    let commission_rates = commission
        .commission_rates
        .ok_or_else(|| StdError::generic_err("the validator's commission rates are undefined"))?;

    Ok(ValidatorResponse {
        operator_address: validator.operator_address,
        jailed: validator.jailed,
        moniker: description.moniker,
        identity: description.identity,
        commission_rate: Decimal::from_str(&commission_rates.rate)?,
    })
}
