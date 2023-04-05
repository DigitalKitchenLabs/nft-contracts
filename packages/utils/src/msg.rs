use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Addr};
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
    pub mint_price: Coin,
    //This ratio will be burnt
    pub burn_ratio: Option<u32>,
    //Rest sent here
    pub destination: Option<Addr>,
}

/// Message for params so they can be updated individually by governance
#[cw_serde]
pub struct UpdateManagerParamsMsg {
    pub mint_price: Coin,
    //This ratio will be burnt
    pub burn_ratio: Option<u32>,
    //Rest sent here
    pub destination: Option<Addr>,
}