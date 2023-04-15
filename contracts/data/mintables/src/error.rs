use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    
    #[error(transparent)]
    Ownership(#[from] OwnershipError),

    #[error("ID already exists, cannot add")]
    IDExists {},

    #[error("Goods and possibilities arrays must be the same length")]
    NotSameLength {},

    #[error("Goods cannot be empty")]
    Empty {},

    #[error("Sum of lootbox probabilities must be 100")]
    InvalidProbabilities {},
}
