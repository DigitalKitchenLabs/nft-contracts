
use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;


#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("If mint price is not fully burned, destination address cannot be empty")]
    NoMintDestination {},
}