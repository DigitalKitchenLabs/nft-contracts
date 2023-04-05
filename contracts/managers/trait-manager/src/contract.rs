use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Empty};
use cw2::set_contract_version;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use utils::msg::BaseManagerCreateMsg;
use crate::ContractError;


const CONTRACT_NAME: &str = "crates.io:sg-base-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_CW721_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: BaseManagerCreateMsg<Option<Empty>>,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}