use crate::{
    msg::ExecuteMsg,
    state::{
        increment_token_index, Config, COLLECTION_ADDRESS, CONFIG, MINTABLE_COLLECTION_ADDRESS,
    },
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
use mintables::msg::{QueryMsg, TraitBundlesResp, TraitLootboxesResp, TraitsResp};
use sha2::{Digest, Sha256};
use utils::{
    msg::{BaseTraitManagerCreateMsg, UpdateTraitManagerParamsMsg},
    query::{AllowedCollectionCodeIdResponse, TraitManagerQueryMsg, TraitManagerConfigResponse},
    U64Ext, NATIVE_DENOM,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
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
    if msg
        .manager_params
        .mint_prices
        .iter()
        .any(|coin| coin.denom != NATIVE_DENOM)
        && msg.manager_params.destination.is_none()
    {
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

    //Store the address of the mintables collection for future queries
    deps.api.addr_validate(
        &msg.manager_params
            .mintable_collection_addr
            .clone()
            .into_string(),
    )?;

    MINTABLE_COLLECTION_ADDRESS.save(deps.storage, &msg.manager_params.mintable_collection_addr)?;

    let config = Config {
        collection_code_id: msg.collection_params.code_id,
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
            symbol: msg.collection_params.symbol.unwrap(),
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
        ExecuteMsg::MintBundle {
            bundle_id,
            receiver,
        } => mint_bundle(deps, info, bundle_id, receiver),
        ExecuteMsg::OpenLootbox {
            lootbox_id,
            receiver,
        } => open_lootbox(deps, info, env, lootbox_id, receiver),
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

    //We check if the trait is mintable
    let mintables_collection_address = MINTABLE_COLLECTION_ADDRESS.load(deps.storage)?;
    let traits_response: TraitsResp = deps
        .querier
        .query_wasm_smart(mintables_collection_address, &QueryMsg::Traits {})?;

    let new_trait = traits_response.traits.iter().find(|t| {
        t.trait_type == token_info.trait_type
            && t.trait_value == token_info.trait_value
            && t.trait_rarity == token_info.trait_rarity
    });

    if new_trait.is_none() {
        return Err(ContractError::InvalidTrait {});
    }

    let config = CONFIG.load(deps.storage)?;
    let mut res = Response::new();

    if funds_sent != new_trait.unwrap().mint_price {
        return Err(ContractError::IncorrectMintFunds {});
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
    } else {
        //Send full amount as nothing is burnt.
        let send_funds_msg = BankMsg::Send {
            to_address: config.destination.unwrap().into_string(),
            amount: coins(funds_sent.amount.u128(), funds_sent.denom),
        };
        res.messages.push(SubMsg::new(send_funds_msg))
    }

    let collection_address = COLLECTION_ADDRESS.load(deps.storage)?;

    // Create mint msgs
    let mint_msg = cw721_trait_onchain::ExecuteMsg::<Extension, Empty>::Mint {
        token_id: increment_token_index(deps.storage)?.to_string(),
        owner: receiver.clone(),
        token_uri: None,
        extension: token_info,
    };

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

pub fn mint_bundle(
    deps: DepsMut,
    info: MessageInfo,
    bundle_id: u32,
    receiver: Option<String>,
) -> Result<Response, ContractError> {
    let send_to = receiver.unwrap_or(info.sender.to_string());
    deps.api.addr_validate(&send_to)?;

    let funds_sent = one_coin(&info)?;

    //We check if the bundle is mintable
    let mintables_collection_address = MINTABLE_COLLECTION_ADDRESS.load(deps.storage)?;
    let bundle_response: TraitBundlesResp = deps
        .querier
        .query_wasm_smart(mintables_collection_address, &QueryMsg::TraitBundles {})?;

    let bundle = bundle_response.bundles.iter().find(|b| b.id == bundle_id);

    if bundle.is_none() {
        return Err(ContractError::InvalidBundle {});
    }

    let config = CONFIG.load(deps.storage)?;
    let mut res = Response::new();

    if funds_sent != bundle.unwrap().mint_price {
        return Err(ContractError::IncorrectMintFunds {});
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
    } else {
        //Send full amount as nothing is burnt.
        let send_funds_msg = BankMsg::Send {
            to_address: config.destination.unwrap().into_string(),
            amount: coins(funds_sent.amount.u128(), funds_sent.denom),
        };
        res.messages.push(SubMsg::new(send_funds_msg))
    }

    let collection_address = COLLECTION_ADDRESS.load(deps.storage)?;

    for new_trait in bundle.unwrap().traits.clone() {
        let token_info = Extension {
            trait_type: new_trait.trait_type,
            trait_rarity: new_trait.trait_rarity,
            trait_value: new_trait.trait_value,
        };

        // Create mint msgs
        let mint_msg = cw721_trait_onchain::ExecuteMsg::<Extension, Empty>::Mint {
            token_id: increment_token_index(deps.storage)?.to_string(),
            owner: send_to.clone(),
            token_uri: None,
            extension: token_info,
        };

        let msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: collection_address.to_string(),
            msg: to_binary(&mint_msg)?,
            funds: vec![],
        });
        res = res.add_message(msg);
    }

    Ok(res
        .add_attribute("action", "mint_bundle")
        .add_attribute("bundle_id", bundle_id.to_string())
        .add_attribute("sender", info.sender)
        .add_attribute("receiver", send_to))
}

pub fn open_lootbox(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    lootbox_id: u32,
    receiver: Option<String>,
) -> Result<Response, ContractError> {
    let send_to = receiver.unwrap_or(info.sender.to_string());
    deps.api.addr_validate(&send_to)?;

    let funds_sent = one_coin(&info)?;

    //We check if the bundle is mintable
    let mintables_collection_address = MINTABLE_COLLECTION_ADDRESS.load(deps.storage)?;
    let lootbox_response: TraitLootboxesResp = deps
        .querier
        .query_wasm_smart(mintables_collection_address, &QueryMsg::TraitLootboxes {})?;

    let lootbox = lootbox_response
        .lootboxes
        .iter()
        .find(|lb| lb.id == lootbox_id);

    if lootbox.is_none() {
        return Err(ContractError::InvalidLootbox {});
    }

    let config = CONFIG.load(deps.storage)?;
    let mut res = Response::new();

    if funds_sent != lootbox.unwrap().mint_price {
        return Err(ContractError::IncorrectMintFunds {});
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
    } else {
        //Send full amount as nothing is burnt.
        let send_funds_msg = BankMsg::Send {
            to_address: config.destination.unwrap().into_string(),
            amount: coins(funds_sent.amount.u128(), funds_sent.denom),
        };
        res.messages.push(SubMsg::new(send_funds_msg))
    }

    let collection_address = COLLECTION_ADDRESS.load(deps.storage)?;
    let mut current = random_number_1_to_x(
        &env,
        send_to.clone(),
        lootbox_response.lootboxes.len().try_into().unwrap(),
        100
    );
    let mut position = 0;

    //Find which item of the lootbox we get according to possibilities
    for possibility in lootbox.unwrap().possibilities.clone() {
        if current <= possibility {
            break;
        } else {
            position += 1;
            current -= possibility;
        }
    }

    let token_info = Extension {
        trait_type: lootbox.unwrap().traits[position].clone().trait_type,
        trait_rarity: lootbox.unwrap().traits[position].clone().trait_rarity,
        trait_value: lootbox.unwrap().traits[position].clone().trait_value,
    };

    // Create mint msgs
    let mint_msg = cw721_trait_onchain::ExecuteMsg::<Extension, Empty>::Mint {
        token_id: increment_token_index(deps.storage)?.to_string(),
        owner: send_to.clone(),
        token_uri: None,
        extension: token_info,
    };

    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: collection_address.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    });

    res = res.add_message(msg);

    Ok(res
        .add_attribute("action", "open_lootbox")
        .add_attribute("lootbox_id", lootbox_id.to_string())
        .add_attribute("won_element", position.to_string())
        .add_attribute("sender", info.sender)
        .add_attribute("receiver", send_to))
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

    let range = 0..100;
    if !range.contains(&new_config.burn_ratio) {
        return Err(ContractError::InvalidBurnRatio {});
    }

    //If we are selling anything using a non native denom, we need a destination address as we will not burn those non native tokens.
    if new_config
        .mint_prices
        .iter()
        .any(|coin| coin.denom != NATIVE_DENOM)
        && new_config.destination.is_none()
    {
        return Err(ContractError::NoMintDestination {});
    }

    //The mint prices and rarities arrays must be same length, 1-to-1 price/rarity
    if new_config.mint_prices.len() != new_config.rarities.len() {
        return Err(ContractError::NotSameLength {});
    }

    let mut config = CONFIG.load(deps.storage)?;
    config.burn_ratio = new_config.burn_ratio;
    config.destination = new_config.destination;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: TraitManagerQueryMsg) -> StdResult<Binary> {
    match msg {
        TraitManagerQueryMsg::Config {} => to_binary(&query_config(deps)?),
        TraitManagerQueryMsg::AllowedCollectionCodeId {} => to_binary(&query_codeid(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<TraitManagerConfigResponse<Empty>> {
    let config = CONFIG.load(deps.storage)?;
    let collection_address = COLLECTION_ADDRESS.load(deps.storage)?;

    Ok(TraitManagerConfigResponse {
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

//We get around using random libraries by importing the things we need from
//https://docs.rs/rand/0.8.1/i686-unknown-linux-gnu/src/rand/rngs/xoshiro128plusplus.rs.html

fn random_number_1_to_x(env: &Env, sender: String, array_len: u32, x: u32) -> u32 {
    let sha256 = Sha256::digest(
        format!("{}{}{}{}", sender, env.block.time.nanos(), env.block.height, array_len).into_bytes(),
    );
    // Cut first 16 bytes from 32 byte value
    let randomness: [u8; 16] = sha256.to_vec()[0..16].try_into().unwrap();
    let mut state = [0; 4];
    read_u32_into(&randomness, &mut state);
    let rng = get_u32(&mut state);
    let a_number = rng.checked_rem_euclid(x).unwrap() + 1;

    a_number
}

pub fn read_u32_into(src: &[u8], dst: &mut [u32]) {
    assert!(src.len() >= 4 * dst.len());
    for (out, chunk) in dst.iter_mut().zip(src.chunks_exact(4)) {
        *out = u32::from_le_bytes(chunk.try_into().unwrap());
    }
}

fn get_u32(dst: &mut [u32]) -> u32 {
    let result_starstar = dst[0]
        .wrapping_add(dst[3])
        .rotate_left(7)
        .wrapping_add(dst[0]);

    let t = dst[1] << 9;

    dst[2] ^= dst[0];
    dst[3] ^= dst[1];
    dst[1] ^= dst[2];
    dst[0] ^= dst[3];

    dst[2] ^= t;

    dst[3] = dst[3].rotate_left(11);

    result_starstar
}
