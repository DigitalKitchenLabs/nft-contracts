use character_manager::msg::ExecuteMsg;
use cosmwasm_std::Empty;
use utils::msg::BaseCharacterManagerCreateMsg as InstantiateMsg;
use utils::query::CharacterManagerQueryMsg as QueryMsg;
use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        instantiate: InstantiateMsg<Empty>,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}