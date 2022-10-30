use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
use cosmwasm_std::{Response};
use crate::error::ContractError;

#[entry_point]
pub fn instantiate() -> Result<Response, ContractError> {
    Ok(Response::default())
}



