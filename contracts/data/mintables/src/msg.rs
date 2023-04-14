use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;

use crate::state::Character;
use crate::state::CharacterBundle;
use crate::state::CharacterLootbox;
use crate::state::Trait;
use crate::state::TraitBundle;
use crate::state::TraitLootbox;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TraitsResp)]
    Traits {},
    #[returns(CharactersResp)]
    Characters {},
    #[returns(TraitBundlesResp)]
    TraitBundles {},
    #[returns(CharacterBundlesResp)]
    CharacterBundles {},
    #[returns(TraitLootboxesResp)]
    TraitLootboxes {},
    #[returns(CharacterLootboxesResp)]
    CharacterLootboxes {},
}

#[cw_serde]
pub struct TraitsResp {
    pub traits: Vec<Trait>,
}

#[cw_serde]
pub struct CharactersResp {
    pub characters: Vec<Character>,
}

#[cw_serde]
pub struct TraitBundlesResp {
    pub bundles: Vec<TraitBundle>,
}
#[cw_serde]
pub struct CharacterBundlesResp {
    pub bundles: Vec<CharacterBundle>,
}
#[cw_serde]
pub struct TraitLootboxesResp {
    pub lootboxes: Vec<TraitLootbox>,
}
#[cw_serde]
pub struct CharacterLootboxesResp {
    pub lootboxes: Vec<CharacterLootbox>,
}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddTraits {
        new_traits: Vec<Trait>,
    },
    RemoveTraits {
        ids: Vec<u32>,
    },
    AddCharacters {
        new_characters: Vec<Character>,
    },
    RemoveCharacters {
        ids: Vec<u32>,
    },
    AddTraitBundles {
        new_trait_bundles: Vec<TraitBundle>,
    },
    RemoveTraitBundles {
        ids: Vec<u32>,
    },
    AddCharacterBundles {
        new_character_bundles: Vec<CharacterBundle>,
    },
    RemoveCharacterBundles {
        ids: Vec<u32>,
    },
    AddTraitLootboxes {
        new_trait_lootboxes: Vec<TraitLootbox>,
    },
    RemoveTraitLootboxes {
        ids: Vec<u32>,
    },
    AddCharacterLootboxes {
        new_character_lootboxes: Vec<CharacterLootbox>,
    },
    RemoveCharacterLootboxes {
        ids: Vec<u32>,
    },
}
