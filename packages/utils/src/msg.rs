use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw721_base::msg::{CollectionInfo, RoyaltyInfoResponse};

pub type BaseManagerCreateMsg<T> = CreateManagerMsg<T>;

#[cw_serde]
pub struct CreateManagerMsg<T> {
    pub init_msg: T,
    pub collection_params: CollectionParams,
    pub manager_params: ManagerParams,
}

#[cw_serde]
pub struct CollectionParams {
    /// The collection code id
    pub code_id: u64,
    pub name: String,
    pub symbol: String,
    pub info: CollectionInfo<RoyaltyInfoResponse>,
}

#[cw_serde]
pub struct ManagerParams {
    pub mint_prices: Vec<Coin>,
    pub rarities: Vec<String>,
    //This ratio will be burnt
    pub burn_ratio: u64,
    //Rest sent here
    pub destination: Option<Addr>,
}

#[cw_serde]
pub struct UpdateManagerParamsMsg {
    pub mint_prices: Vec<Coin>,
    pub rarities: Vec<String>,
    //This ratio will be burnt
    pub burn_ratio: u64,
    //Rest sent here
    pub destination: Option<Addr>,
}
