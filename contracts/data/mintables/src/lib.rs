mod contract;
pub mod error;
pub mod msg;
mod state;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use error::ContractError;
use msg::{ExecuteMsg, InstantiateMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        Traits {} => to_binary(&query::traits(deps)?),
        Characters {} => to_binary(&query::characters(deps)?),
        TraitBundles {} => to_binary(&query::trait_bundles(deps)?),
        CharacterBundles {} => to_binary(&query::character_bundles(deps)?),
        TraitLootboxes {} => to_binary(&query::trait_lootboxes(deps)?),
        CharacterLootboxes {} => to_binary(&query::character_lootboxes(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use contract::exec::*;

    match msg {
        ExecuteMsg::AddTraits { new_traits } => add_traits(deps, info, new_traits),
        ExecuteMsg::RemoveTraits { ids } => remove_traits(deps, info, ids),
        ExecuteMsg::AddCharacters { new_characters } => add_characters(deps, info, new_characters),
        ExecuteMsg::RemoveCharacters { ids } => remove_characters(deps, info, ids),
        ExecuteMsg::AddTraitBundles { new_trait_bundles } => {
            add_trait_bundles(deps, info, new_trait_bundles)
        }
        ExecuteMsg::RemoveTraitBundles { ids } => remove_trait_bundles(deps, info, ids),
        ExecuteMsg::AddCharacterBundles {
            new_character_bundles,
        } => add_character_bundles(deps, info, new_character_bundles),
        ExecuteMsg::RemoveCharacterBundles { ids } => remove_character_bundles(deps, info, ids),
        ExecuteMsg::AddTraitLootboxes {
            new_trait_lootboxes,
        } => add_trait_lootboxes(deps, info, new_trait_lootboxes),
        ExecuteMsg::RemoveTraitLootboxes { ids } => remove_trait_lootboxes(deps, info, ids),
        ExecuteMsg::AddCharacterLootboxes {
            new_character_lootboxes,
        } => add_character_lootboxes(deps, info, new_character_lootboxes),
        ExecuteMsg::RemoveCharacterLootboxes { ids } => remove_character_lootboxes(deps, info, ids),
    }
}
