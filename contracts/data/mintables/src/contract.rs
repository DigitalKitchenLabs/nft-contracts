use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::state::{
    CHARACTERS, CHARACTER_BUNDLES, CHARACTER_LOOTBOXES, TRAITS, TRAIT_BUNDLES, TRAIT_LOOTBOXES,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(
        deps.storage,
        deps.api,
        Some(&info.sender.clone().into_string()),
    )?;
    TRAITS.save(deps.storage, &vec![])?;
    CHARACTERS.save(deps.storage, &vec![])?;
    TRAIT_BUNDLES.save(deps.storage, &vec![])?;
    CHARACTER_BUNDLES.save(deps.storage, &vec![])?;
    TRAIT_LOOTBOXES.save(deps.storage, &vec![])?;
    CHARACTER_LOOTBOXES.save(deps.storage, &vec![])?;

    Ok(Response::new()
        .add_attribute("action", "Instantiating mintables contract")
        .add_attribute("owner", info.sender))
}

pub mod exec {
    use cosmwasm_std::{DepsMut, MessageInfo, Response};

    use crate::{
        error::ContractError,
        state::{
            Character, CharacterBundle, CharacterLootbox, Trait, TraitBundle, TraitLootbox,
            CHARACTERS, CHARACTER_BUNDLES, CHARACTER_LOOTBOXES, TRAITS, TRAIT_BUNDLES,
            TRAIT_LOOTBOXES,
        },
    };

    pub fn add_traits(
        deps: DepsMut,
        info: MessageInfo,
        new_traits: Vec<Trait>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut traits = TRAITS.load(deps.storage)?;

        for new_trait in new_traits {
            if traits.iter().any(|t| t.id == new_trait.id) {
                return Err(ContractError::IDExists {});
            }
            traits.push(new_trait)
        }

        TRAITS.save(deps.storage, &traits)?;

        Ok(Response::new().add_attribute("action", "add_traits"))
    }

    pub fn remove_traits(
        deps: DepsMut,
        info: MessageInfo,
        trait_ids: Vec<u32>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut traits = TRAITS.load(deps.storage)?;

        for trait_id in trait_ids {
            traits.retain(|t| t.id != trait_id);
        }

        TRAITS.save(deps.storage, &traits)?;

        Ok(Response::new().add_attribute("action", "remove_traits"))
    }

    pub fn add_characters(
        deps: DepsMut,
        info: MessageInfo,
        new_characters: Vec<Character>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut characters = CHARACTERS.load(deps.storage)?;

        for new_character in new_characters {
            if characters.iter().any(|c| c.id == new_character.id) {
                return Err(ContractError::IDExists {});
            }
            characters.push(new_character)
        }

        CHARACTERS.save(deps.storage, &characters)?;

        Ok(Response::new().add_attribute("action", "add_characters"))
    }

    pub fn remove_characters(
        deps: DepsMut,
        info: MessageInfo,
        character_ids: Vec<u32>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut characters = CHARACTERS.load(deps.storage)?;

        for character_id in character_ids {
            characters.retain(|c| c.id != character_id);
        }

        CHARACTERS.save(deps.storage, &characters)?;

        Ok(Response::new().add_attribute("action", "remove_characters"))
    }

    pub fn add_trait_bundles(
        deps: DepsMut,
        info: MessageInfo,
        new_trait_bundles: Vec<TraitBundle>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut trait_bundles = TRAIT_BUNDLES.load(deps.storage)?;

        for new_trait_bundle in new_trait_bundles {
            if trait_bundles.iter().any(|tb| tb.id == new_trait_bundle.id) {
                return Err(ContractError::IDExists {});
            }
            trait_bundles.push(new_trait_bundle)
        }

        TRAIT_BUNDLES.save(deps.storage, &trait_bundles)?;

        Ok(Response::new().add_attribute("action", "add_trait_bundles"))
    }

    pub fn remove_trait_bundles(
        deps: DepsMut,
        info: MessageInfo,
        trait_bundle_ids: Vec<u32>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut trait_bundles = TRAIT_BUNDLES.load(deps.storage)?;

        for trait_bundle_id in trait_bundle_ids {
            trait_bundles.retain(|tb| tb.id != trait_bundle_id);
        }

        TRAIT_BUNDLES.save(deps.storage, &trait_bundles)?;

        Ok(Response::new().add_attribute("action", "remove_trait_bundles"))
    }

    pub fn add_character_bundles(
        deps: DepsMut,
        info: MessageInfo,
        new_character_bundles: Vec<CharacterBundle>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut character_bundles = CHARACTER_BUNDLES.load(deps.storage)?;

        for new_character_bundle in new_character_bundles {
            if character_bundles
                .iter()
                .any(|cb| cb.id == new_character_bundle.id)
            {
                return Err(ContractError::IDExists {});
            }
            character_bundles.push(new_character_bundle)
        }

        CHARACTER_BUNDLES.save(deps.storage, &character_bundles)?;

        Ok(Response::new().add_attribute("action", "add_character_bundles"))
    }

    pub fn remove_character_bundles(
        deps: DepsMut,
        info: MessageInfo,
        character_bundle_ids: Vec<u32>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut character_bundles = CHARACTER_BUNDLES.load(deps.storage)?;

        for character_bundle_id in character_bundle_ids {
            character_bundles.retain(|cb| cb.id != character_bundle_id);
        }

        CHARACTER_BUNDLES.save(deps.storage, &character_bundles)?;

        Ok(Response::new().add_attribute("action", "remove_character_bundles"))
    }

    pub fn add_trait_lootboxes(
        deps: DepsMut,
        info: MessageInfo,
        new_trait_lootboxes: Vec<TraitLootbox>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut trait_lootboxes = TRAIT_LOOTBOXES.load(deps.storage)?;

        for new_trait_lootbox in new_trait_lootboxes {
            if trait_lootboxes
                .iter()
                .any(|tl| tl.id == new_trait_lootbox.id)
            {
                return Err(ContractError::IDExists {});
            }
            trait_lootboxes.push(new_trait_lootbox)
        }

        TRAIT_LOOTBOXES.save(deps.storage, &trait_lootboxes)?;

        Ok(Response::new().add_attribute("action", "add_trait_lootboxes"))
    }

    pub fn remove_trait_lootboxes(
        deps: DepsMut,
        info: MessageInfo,
        trait_lootbox_ids: Vec<u32>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut trait_lootboxes = TRAIT_LOOTBOXES.load(deps.storage)?;

        for trait_lootbox_id in trait_lootbox_ids {
            trait_lootboxes.retain(|tl| tl.id != trait_lootbox_id);
        }

        TRAIT_LOOTBOXES.save(deps.storage, &trait_lootboxes)?;

        Ok(Response::new().add_attribute("action", "remove_trait_lootboxes"))
    }

    pub fn add_character_lootboxes(
        deps: DepsMut,
        info: MessageInfo,
        new_character_lootboxes: Vec<CharacterLootbox>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut character_lootboxes = CHARACTER_LOOTBOXES.load(deps.storage)?;

        for new_character_lootbox in new_character_lootboxes {
            if character_lootboxes
                .iter()
                .any(|cl| cl.id == new_character_lootbox.id)
            {
                return Err(ContractError::IDExists {});
            }
            character_lootboxes.push(new_character_lootbox)
        }

        CHARACTER_LOOTBOXES.save(deps.storage, &character_lootboxes)?;

        Ok(Response::new().add_attribute("action", "add_character_lootboxes"))
    }

    pub fn remove_character_lootboxes(
        deps: DepsMut,
        info: MessageInfo,
        character_lootbox_ids: Vec<u32>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut character_lootboxes = CHARACTER_LOOTBOXES.load(deps.storage)?;

        for character_lootbox_id in character_lootbox_ids {
            character_lootboxes.retain(|cl| cl.id != character_lootbox_id);
        }

        CHARACTER_LOOTBOXES.save(deps.storage, &character_lootboxes)?;

        Ok(Response::new().add_attribute("action", "remove_character_lootboxes"))
    }
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};

    use crate::{
        msg::{
            CharacterBundlesResp, CharacterLootboxesResp, CharactersResp, TraitBundlesResp,
            TraitLootboxesResp, TraitsResp,
        },
        state::{
            CHARACTERS, CHARACTER_BUNDLES, CHARACTER_LOOTBOXES, TRAITS, TRAIT_BUNDLES,
            TRAIT_LOOTBOXES,
        },
    };

    pub fn traits(deps: Deps) -> StdResult<TraitsResp> {
        let traits = TRAITS.load(deps.storage)?;
        Ok(TraitsResp { traits })
    }

    pub fn characters(deps: Deps) -> StdResult<CharactersResp> {
        let characters = CHARACTERS.load(deps.storage)?;
        Ok(CharactersResp { characters })
    }

    pub fn trait_bundles(deps: Deps) -> StdResult<TraitBundlesResp> {
        let bundles = TRAIT_BUNDLES.load(deps.storage)?;
        Ok(TraitBundlesResp { bundles })
    }

    pub fn character_bundles(deps: Deps) -> StdResult<CharacterBundlesResp> {
        let bundles = CHARACTER_BUNDLES.load(deps.storage)?;
        Ok(CharacterBundlesResp { bundles })
    }

    pub fn trait_lootboxes(deps: Deps) -> StdResult<TraitLootboxesResp> {
        let lootboxes = TRAIT_LOOTBOXES.load(deps.storage)?;
        Ok(TraitLootboxesResp { lootboxes })
    }

    pub fn character_lootboxes(deps: Deps) -> StdResult<CharacterLootboxesResp> {
        let lootboxes = CHARACTER_LOOTBOXES.load(deps.storage)?;
        Ok(CharacterLootboxesResp { lootboxes })
    }
}
