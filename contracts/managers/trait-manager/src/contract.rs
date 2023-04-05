use crate::{
    state::{Config, CONFIG},
    ContractError,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, DepsMut, Empty, Env, MessageInfo, Response, SubMsg, WasmMsg};
use cw2::set_contract_version;
use cw721_trait_onchain::InstantiateMsg;
use utils::msg::BaseManagerCreateMsg;

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

    //If mint price is not fully burned then there must be a valid destination address to send funds to
    if msg.manager_params.burn_ratio.unwrap() != 100 && msg.manager_params.destination.is_none() {
        return Err(ContractError::NoMintDestination {});
    }

    if msg.manager_params.burn_ratio.unwrap() != 100 {
        deps.api.addr_validate(
            &msg.manager_params
                .destination
                .clone()
                .unwrap()
                .into_string(),
        )?;
    }

    let config = Config {
        collection_code_id: msg.collection_params.code_id,
        mint_price: msg.manager_params.mint_price,
        burn_ratio: msg.manager_params.burn_ratio,
        destination: msg.manager_params.destination,
        extension: Empty {},
    };

    CONFIG.save(deps.storage, &config)?;

    let collection_info = msg.collection_params.info.clone();

    let wasm_msg = WasmMsg::Instantiate {
        code_id: msg.collection_params.code_id,
        msg: to_binary(&InstantiateMsg {
            name: msg.collection_params.name.clone(),
            symbol: msg.collection_params.symbol,
            minter: env.contract.address.to_string(),
            collection_info,
        })?,
        funds: info.funds,
        admin: None,
        label: format!(
            "CW721-TRAIT-COLLECTION--{}-{}",
            msg.collection_params.code_id,
            msg.collection_params.name.trim()
        ),
    };
    let submsg = SubMsg::reply_on_success(wasm_msg, INSTANTIATE_CW721_REPLY_ID);

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION)
        .add_attribute("sender", info.sender)
        .add_submessage(submsg))
}
