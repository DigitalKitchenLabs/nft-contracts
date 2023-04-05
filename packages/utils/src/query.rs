use cosmwasm_schema::cw_serde;

use crate::CodeId;
use crate::ManagerParams;

#[cw_serde]
pub enum ManagerQueryMsg {
    /// Returns `ParamsResponse`
    Params {},
    AllowedCollectionCodeId {},
}

#[cw_serde]
pub struct ParamsResponse<T> {
    pub params: ManagerParams<T>,
}

#[cw_serde]
pub struct AllowedCollectionCodeIdResponse {
    pub code_id: CodeId,
}