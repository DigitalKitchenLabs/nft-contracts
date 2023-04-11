use cosmwasm_schema::cw_serde;
use cw721_character_onchain::msg::Metadata;
use cw_ownable::cw_ownable_execute;
use utils::msg::UpdateCharacterManagerParamsMsg;

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
    ModifyCharacter {
        token_id: String,
        trait_ids: Vec<String>,
        trait_collection_addr: String,
    },
    UpdateConfig {
        new_config: UpdateCharacterManagerParamsMsg,
    },
}
