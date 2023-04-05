use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};

pub type BaseManagerCreateMsg<T> = CreateManagerMsg<T>;

#[cw_serde]
pub struct CreateManagerMsg<T> {
    pub init_msg: T,
    pub collection_params: CollectionParams,
    pub manager_params: ManagerParams,
}

#[cw_serde]
pub struct CollectionInfo<T> {
    pub creator: String,
    pub description: String,
    pub image: String,
    pub external_link: Option<String>,
    pub explicit_content: Option<bool>,
    pub royalty_info: Option<T>,
}

#[cw_serde]
pub struct RoyaltyInfoResponse {
    pub payment_address: String,
    pub share: Decimal,
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
    pub creation_fee: Option<Coin>,
    pub min_mint_price: Option<Coin>,
    pub mint_fee_bps: Option<u64>,
}

/// Message for params so they can be updated individually by governance
#[cw_serde]
pub struct UpdateManagerParamsMsg {
    pub creation_fee: Option<Coin>,
    pub min_mint_price: Option<Coin>,
    pub mint_fee_bps: Option<u64>,
}