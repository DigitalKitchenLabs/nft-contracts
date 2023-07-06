use cosmwasm_schema::QueryResponses;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;

use crate::CharacterManagerConfig;
use crate::CodeId;
use crate::TraitManagerConfig;

#[cw_serde]
#[derive(QueryResponses)]
pub enum CharacterManagerQueryMsg {
    #[returns(CharacterManagerConfigResponse<Empty>)]
    Config {},
    #[returns(AllowedCollectionCodeIdResponse)]
    AllowedCollectionCodeId {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum TraitManagerQueryMsg {
    #[returns(TraitManagerConfigResponse<Empty>)]
    Config {},
    #[returns(AllowedCollectionCodeIdResponse)]
    AllowedCollectionCodeId {},
}

#[cw_serde]
pub struct TraitManagerConfigResponse<T> {
    pub collection_address: String,
    pub config: TraitManagerConfig<T>,
}

#[cw_serde]
pub struct CharacterManagerConfigResponse<T> {
    pub collection_address: String,
    pub config: CharacterManagerConfig<T>,
}

#[cw_serde]
pub struct AllowedCollectionCodeIdResponse {
    pub code_id: CodeId,
}
