use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw721_base::msg::{CollectionInfo, RoyaltyInfoResponse};

pub type BaseTraitManagerCreateMsg<T> = CreateTraitManagerMsg<T>;
pub type BaseCharacterManagerCreateMsg<T> = CreateCharacterManagerMsg<T>;

#[cw_serde]
pub struct CollectionParams {
    /// The collection code id
    pub code_id: u64,
    pub name: String,
    pub symbol: Option<String>,
    pub info: CollectionInfo<RoyaltyInfoResponse>,
}

#[cw_serde]
pub struct CreateTraitManagerMsg<T> {
    pub init_msg: T,
    pub collection_params: CollectionParams,
    pub manager_params: TraitManagerParams,
}

#[cw_serde]
pub struct TraitManagerParams {
    pub mint_prices: Vec<Coin>,
    pub rarities: Vec<String>,
    //This ratio will be burnt
    pub burn_ratio: u64,
    //Rest sent here
    pub destination: Option<Addr>,
}

#[cw_serde]
pub struct UpdateTraitManagerParamsMsg {
    pub mint_prices: Vec<Coin>,
    pub rarities: Vec<String>,
    //This ratio will be burnt
    pub burn_ratio: u64,
    //Rest sent here
    pub destination: Option<Addr>,
}

#[cw_serde]
pub struct CreateCharacterManagerMsg<T> {
    pub init_msg: T,
    pub collection_params: CollectionParams,
    pub manager_params: CharacterManagerParams,
}

#[cw_serde]
pub struct CharacterManagerParams {
    pub empty_character_mint_price: Coin,
    pub character_mint_prices: Vec<Coin>,
    pub character_rarities: Vec<String>,
    //This ratio will be burnt
    pub burn_ratio: u64,
    //Rest sent here
    pub destination: Option<Addr>,
    pub trait_collection_addr: Addr,
    pub mintable_collection_addr: Addr,
}

#[cw_serde]
pub struct UpdateCharacterManagerParamsMsg {
    pub empty_character_mint_price: Coin,
    pub character_mint_prices: Vec<Coin>,
    pub character_rarities: Vec<String>,
    //This ratio will be burnt
    pub burn_ratio: u64,
    //Rest sent here
    pub destination: Option<Addr>,
}
