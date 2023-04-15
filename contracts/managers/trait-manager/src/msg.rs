use cosmwasm_schema::cw_serde;
use cw721_trait_onchain::msg::Metadata;
use cw_ownable::cw_ownable_execute;
use utils::msg::UpdateTraitManagerParamsMsg;

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    Mint {
        token_info: Metadata,
    },
    MintTo {
        token_info: Metadata,
        receiver: String,
    },
    MintBundle{
        bundle_id: u32,
        receiver: Option<String>,
    },
    UpdateConfig {
        new_config: UpdateTraitManagerParamsMsg,
    },
}
