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

    #[error("Rarity not found")]
    InvalidRarity {},

    #[error("Not enough funds sent to mint")]
    NotEnoughMintFunds {},

    #[error("If mint price is not fully burned or is not the native currency, destination address of mint funds cannot be empty")]
    NoMintDestination {},
}
