use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

pub mod msg;
pub mod query;

pub type CodeId = u64;
pub const NATIVE_DENOM: &str = "uccat";

/// Common params for all minters used for storage
#[cw_serde]
pub struct TraitManagerConfig<T> {
    pub collection_code_id: u64,
    pub burn_ratio: u64,
    pub destination: Option<Addr>,
    pub extension: T,
}

#[cw_serde]
pub struct CharacterManagerConfig<T> {
    pub collection_code_id: u64,
    pub empty_character_mint_price: Coin,
    //This ratio will be burnt
    pub burn_ratio: u64,
    //Rest sent here
    pub destination: Option<Addr>,
    pub extension: T,
}

pub trait U64Ext {
    fn bps_to_decimal(self) -> Decimal;
}

impl U64Ext for u64 {
    fn bps_to_decimal(self) -> Decimal {
        Decimal::percent(self) / Uint128::from(100u128)
    }
}
