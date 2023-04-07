use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Addr, Decimal, Uint128};

pub mod msg;
pub mod query;

pub type CodeId = u64;
pub const NATIVE_DENOM: &str = "uccat";

/// Common params for all minters used for storage
#[cw_serde]
pub struct ManagerConfig<T> {
    pub collection_code_id: u64,
    pub mint_prices: Vec<Coin>,
    pub rarities: Vec<String>,
    pub burn_ratio: u64,
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