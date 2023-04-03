use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    Ownership(#[from] OwnershipError),

    #[error("{0}")]
    Parse(#[from] ParseError),

    #[error("token_id already claimed")]
    Claimed {},

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },

    #[error("found version ({0}) while attempting to migrate from 0.16.0")]
    WrongMigrateVersion(String),

    #[error("Description of collection is too long")]
    DescriptionTooLong {},

    #[error("CollectionInfoFrozen")]
    CollectionInfoFrozen {},

    #[error("Invalid Royalties")]
    InvalidRoyalties {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("RoyaltyShareIncreased")]
    RoyaltyShareIncreased {},

    #[error("Character is not frozen")]
    CharacterNotFrozen {},

    #[error("Character not found")]
    CharacterNotFound {},

}
