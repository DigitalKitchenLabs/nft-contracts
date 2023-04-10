use crate::{
    msg::ExecuteMsg,
    state::{increment_token_index, Config, COLLECTION_ADDRESS, CONFIG},
    ContractError,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo,
    Reply, Response, StdResult, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw721_trait_onchain::{msg::Extension, InstantiateMsg};
use cw_utils::{one_coin, parse_reply_instantiate_data};
use utils::{
    msg::{BaseTraitManagerCreateMsg, UpdateTraitManagerParamsMsg},
    query::{AllowedCollectionCodeIdResponse, ManagerConfigResponse, ManagerQueryMsg},
    U64Ext, NATIVE_DENOM,
};

const CONTRACT_NAME: &str = "crates.io:sg-base-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_CW721_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: BaseTraitManagerCreateMsg<Option<Empty>>,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    cw_ownable::initialize_owner(
        deps.storage,
        deps.api,
        Some(&info.sender.clone().into_string()),
    )?;

    let range = 0..100;
    if !range.contains(&msg.manager_params.burn_ratio) {
        return Err(ContractError::InvalidBurnRatio {});
    }

    //If mint price is not fully burned then there must be a valid destination address to send funds to
    if msg.manager_params.burn_ratio != 100 && msg.manager_params.destination.is_none() {
        return Err(ContractError::NoMintDestination {});
    }

    //If we are selling anything using a non native denom, we need a destination address as we will not burn those non native tokens.
    if msg.manager_params.mint_prices.iter().any(|coin| coin.denom != NATIVE_DENOM) && msg.manager_params.destination.is_none(){
        return Err(ContractError::NoMintDestination {});
    }

    if msg.manager_params.destination.is_some() {
        deps.api.addr_validate(
            &msg.manager_params
                .destination
                .clone()
                .unwrap()
                .into_string(),
        )?;
    }

    //The mint prices and rarities arrays must be same length, 1-to-1 price/rarity
    if msg.manager_params.mint_prices.len() != msg.manager_params.rarities.len() {
        return Err(ContractError::NotSameLength {});
    }

    let config = Config {
        collection_code_id: msg.collection_params.code_id,
        mint_prices: msg.manager_params.mint_prices,
        rarities: msg.manager_params.rarities,
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { token_info } => {
            mint(deps, info.clone(), token_info, info.sender.into_string())
        }
        ExecuteMsg::MintTo {
            token_info,
            receiver,
        } => mint(deps, info, token_info, receiver),
        ExecuteMsg::UpdateConfig { new_config } => update_config(deps, info, new_config),
        ExecuteMsg::UpdateOwnership(action) => update_ownership(deps, env, info, action),
    }
}

pub fn mint(
    deps: DepsMut,
    info: MessageInfo,
    token_info: Extension,
    receiver: String,
) -> Result<Response, ContractError> {
    deps.api.addr_validate(&receiver)?;

    let funds_sent = one_coin(&info)?;

    let config = CONFIG.load(deps.storage)?;
    let mut res = Response::new();

    let position = config
        .rarities
        .iter()
        .position(|rarity| rarity == &token_info.trait_rarity);

    if position.is_none() {
        return Err(ContractError::InvalidRarity {});
    }

    if funds_sent != config.mint_prices[position.unwrap()] {
        return Err(ContractError::NotEnoughMintFunds {});
    }

    //If we are minting using CoolCat tokens we apply the burn ratio if there is one
    if funds_sent.denom == NATIVE_DENOM && config.burn_ratio > 0 {
        let amount_burnt = config.burn_ratio.bps_to_decimal() * funds_sent.amount;
        let burn_msg = BankMsg::Burn {
            amount: coins(amount_burnt.u128(), NATIVE_DENOM),
        };
        res.messages.push(SubMsg::new(burn_msg));
    }

    //If we are minting using CoolCat we need to adjust the amount sent substracting the burnt amount
    if funds_sent.denom == NATIVE_DENOM && config.burn_ratio != 100 {
        let amount_sent = (100 - config.burn_ratio).bps_to_decimal() * funds_sent.amount;
        let send_funds_msg = BankMsg::Send {
            to_address: config.destination.unwrap().into_string(),
            amount: coins(amount_sent.u128(), funds_sent.denom),
        };
        res.messages.push(SubMsg::new(send_funds_msg));
    }else{
        //Send full amount as nothing is burnt.
        let send_funds_msg = BankMsg::Send {
            to_address: config.destination.unwrap().into_string(),
            amount: coins(funds_sent.amount.u128(), funds_sent.denom),
        };
        res.messages.push(SubMsg::new(send_funds_msg))
    }

    // Create mint msgs
    let mint_msg = cw721_trait_onchain::ExecuteMsg::<Extension, Empty>::Mint {
        token_id: increment_token_index(deps.storage)?.to_string(),
        owner: receiver.clone(),
        token_uri: None,
        extension: token_info,
    };

    let collection_address = COLLECTION_ADDRESS.load(deps.storage)?;

    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: collection_address.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    });
    res = res.add_message(msg);
    Ok(res
        .add_attribute("action", "mint")
        .add_attribute("sender", info.sender)
        .add_attribute("receiver", receiver))
}

pub fn update_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: cw_ownable::Action,
) -> Result<Response, ContractError> {
    let ownership = cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
    Ok(Response::new().add_attributes(ownership.into_attributes()))
}

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    new_config: UpdateTraitManagerParamsMsg,
) -> Result<Response, ContractError> {
    //Only owner can update config
    cw_ownable::assert_owner(deps.storage, &info.sender)?;
    //If mint price is not fully burned then there must be a valid destination address to send funds to
    if new_config.burn_ratio != 100 && new_config.destination.is_none() {
        return Err(ContractError::NoMintDestination {});
    }

    if new_config.burn_ratio != 100 {
        deps.api
            .addr_validate(&new_config.destination.clone().unwrap().into_string())?;
    }

    let mut config = CONFIG.load(deps.storage)?;
    config.mint_prices = new_config.mint_prices;
    config.rarities = new_config.rarities;
    config.burn_ratio = new_config.burn_ratio;
    config.destination = new_config.destination;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn query(deps: Deps, _env: Env, msg: ManagerQueryMsg) -> StdResult<Binary> {
    match msg {
        ManagerQueryMsg::Config {} => to_binary(&query_config(deps)?),
        ManagerQueryMsg::AllowedCollectionCodeId {} => to_binary(&query_codeid(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ManagerConfigResponse<Empty>> {
    let config = CONFIG.load(deps.storage)?;
    let collection_address = COLLECTION_ADDRESS.load(deps.storage)?;

    Ok(ManagerConfigResponse {
        collection_address: collection_address.to_string(),
        config,
    })
}

fn query_codeid(deps: Deps) -> StdResult<AllowedCollectionCodeIdResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(AllowedCollectionCodeIdResponse {
        code_id: config.collection_code_id,
    })
}

// Reply callback triggered from cw721 trait-onchain collection contract instantiation in instantiate()
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id != INSTANTIATE_CW721_REPLY_ID {
        return Err(ContractError::InvalidReplyID {});
    }

    let reply = parse_reply_instantiate_data(msg);
    match reply {
        Ok(res) => {
            let collection_address = res.contract_address;
            COLLECTION_ADDRESS.save(deps.storage, &Addr::unchecked(collection_address.clone()))?;
            Ok(Response::default()
                .add_attribute("action", "instantiate_sg721_reply")
                .add_attribute("cw721_trait_collection_address", collection_address))
        }
        Err(_) => Err(ContractError::InstantiateError {}),
    }
}
