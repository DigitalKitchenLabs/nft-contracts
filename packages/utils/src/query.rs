use cosmwasm_schema::cw_serde;

use crate::CharacterManagerConfig;
use crate::CodeId;
use crate::TraitManagerConfig;

#[cw_serde]
pub enum ManagerQueryMsg {
    /// Returns `ParamsResponse`
    Config {},
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
