use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Addr};

pub mod msg;
pub mod query;

pub type CodeId = u64;

/// Common params for all minters used for storage
#[cw_serde]
pub struct ManagerParams<T> {
    pub cw721_code_id: CodeId,
    pub mint_price: Coin,
    pub extension: T,
}

#[cw_serde]
pub struct ManagerConfig<T> {
    pub collection_code_id: u64,
    pub mint_prices: Vec<Coin>,
    pub rarities: Vec<String>,
    pub burn_ratio: Option<u32>,
    pub destination: Option<Addr>,
    pub extension: T,
}