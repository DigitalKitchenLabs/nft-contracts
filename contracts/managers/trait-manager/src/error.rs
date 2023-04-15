use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error(transparent)]
    Ownership(#[from] OwnershipError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Instantiate cw721 trait collection error")]
    InstantiateError {},

    #[error("Mint prices array must be the same length as rarities array")]
    NotSameLength {},

    #[error("Burn ratio must be between 0 and 100")]
    InvalidBurnRatio {},

    #[error("Incorrect mind funds sent")]
    IncorrectMintFunds {},

    #[error("If mint price is not fully burned or is not the native currency, destination address of mint funds cannot be empty")]
    NoMintDestination {},

    #[error("Trait does not exist in mintables contract")]
    InvalidTrait {},

    #[error("Bundle does not exist in mintables contract")]
    InvalidBundle {},

    #[error("Lootbox does not exist in mintables contract")]
    InvalidLootbox {},
}
