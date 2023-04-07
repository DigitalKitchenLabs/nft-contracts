use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    Mint { token_id: String },
    MintTo { token_id: String, receiver: String }
}