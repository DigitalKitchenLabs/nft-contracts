use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use trait_manager::msg::ExecuteMsg;
use utils::msg::BaseTraitManagerCreateMsg as InstantiateMsg;
use utils::query::TraitManagerQueryMsg as QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg<Empty>,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}