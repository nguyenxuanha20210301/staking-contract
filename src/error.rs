use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Insufficient stake: requested {requested} but available {available}")]
    InsufficientStake { requested: u128, available: u128 },

    #[error("Reward calculation error: {0}")]
    RewardCalculationError(String),

    #[error("Custom Error: {0}")]
    CustomError(String),

    #[error("Std Error: {0}")]
    Std(#[from] StdError),
}
