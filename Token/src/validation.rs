use cosmwasm_std::{Storage, Addr, Uint128, StdResult, BlockInfo};
use crate::error::ContractError;
use crate::state::{TokenConfig, RiskParameters, TX_METADATA, NONCES};

pub fn validate_transaction(
    storage: &dyn Storage,
    sender: &Addr,
    amount: Uint128,
    block_info: &BlockInfo,
    risk_params: &RiskParameters,
) -> Result<(), ContractError> {
    // Check transaction value limits
    if amount > risk_params.max_tx_value {
        return Err(ContractError::TransactionValueTooHigh {});
    }

    // Check daily limits
    let daily_volume = get_daily_volume(storage, sender, block_info.time.seconds())?;
    if daily_volume.checked_add(amount)? > risk_params.daily_limit {
        return Err(ContractError::DailyLimitExceeded {});
    }

    // Validate nonce to prevent replay attacks
    let current_nonce = NONCES.may_load(storage, sender)?.unwrap_or(0);
    if current_nonce >= u64::MAX {
        return Err(ContractError::NonceOverflow {});
    }

    Ok(())
}
