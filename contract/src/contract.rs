use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response,
    StdError, StdResult,
};

use protobuf::Message;

use crate::auth::BaseAccount;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{QueryAccountRequest, QueryAccountResponse};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default()) // do nothing
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response> {
    Err(StdError::generic_err("[cw-stargate-query]: unimplemented"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Sequence {
            address,
        } => to_binary(&query_sequence(deps, address)?),
    }
}

fn query_sequence(deps: Deps, address: String) -> StdResult<u64> {
    // Create request
    let mut req = QueryAccountRequest::new();
    req.set_address(address);
    // Cast request to binary
    let req_bin = req
        .write_to_bytes()
        .map_err(|err| StdError::serialize_err("QueryAccountRequest", err.to_string()))?;

    // Binary response
    let res_bin: Binary = deps.querier.query(&QueryRequest::Stargate {
        path: "/cosmos/auth/v1beta1/accounts/{address}".to_string(),
        data: req_bin.into(),
    })?;
    // Parse response
    let res: QueryAccountResponse = Message::parse_from_bytes(res_bin.as_slice())
        .map_err(|err| StdError::parse_err("QueryAccountResponse", err.to_string()))?;
    let account: BaseAccount = Message::parse_from_bytes(res.get_account().get_value())
        .map_err(|err| StdError::parse_err("BaseAccount", err.to_string()))?;

    Ok(account.get_sequence())
}
