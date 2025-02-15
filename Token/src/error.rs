use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized operation")]
    Unauthorized {},

    #[error("Transaction value exceeds maximum allowed")]
    TransactionValueTooHigh {},

    #[error("Daily transaction limit exceeded")]
    DailyLimitExceeded {},

    #[error("Invalid signature")]
    InvalidSignature {},

    #[error("Nonce overflow")]
    NonceOverflow {},

    #[error("Account in cooling period")]
    CoolingPeriod {},

    #[error("Rate limit exceeded")]
    RateLimitExceeded {},

    #[error("Invalid risk parameters")]
    InvalidRiskParameters {},
}
