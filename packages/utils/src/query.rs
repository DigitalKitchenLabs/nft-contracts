use cosmwasm_schema::cw_serde;

use crate::CodeId;
use crate::ManagerConfig;

#[cw_serde]
pub enum ManagerQueryMsg {
    /// Returns `ParamsResponse`
    Config {},
    AllowedCollectionCodeId {},
}

#[cw_serde]
pub struct ManagerConfigResponse<T> {
    pub collection_address: String,
    pub config: ManagerConfig<T>,
}

#[cw_serde]
pub struct AllowedCollectionCodeIdResponse {
    pub code_id: CodeId,
}