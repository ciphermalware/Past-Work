use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use crate::error::ContractError;
use crate::state::{BALANCES, TOKEN_CONFIG, TX_METADATA, NONCES, RiskParameters};
use crate::validation;
use crate::security::SecurityModule;

pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
    signature: Binary,
) -> Result<Response, ContractError> {
    let config = TOKEN_CONFIG.load(deps.storage)?;
    let risk_params = RISK_PARAMS.load(deps.storage)?;
    
    // Validate transaction
    validation::validate_transaction(
        deps.storage,
        &info.sender,
        amount,
        &env.block,
        &risk_params,
    )?;

    // Verify transaction signature
    let security_module = SecurityModule::new();
    let msg = [info.sender.as_bytes(), recipient.as_bytes(), amount.to_be_bytes().as_ref()].concat();
    if !security_module.verify_signature(&msg, &signature.into()) {
        return Err(ContractError::InvalidSignature {});
    }

    // Update nonce
    let nonce = NONCES.may_load(deps.storage, &info.sender)?.unwrap_or(0);
    NONCES.save(deps.storage, &info.sender, &(nonce + 1))?;

    // Execute transfer
    BALANCES.update(
        deps.storage,
        &info.sender,
        env.block.height,
        |balance: Option<Uint128>| -> StdResult<_> {
            let balance = balance.unwrap_or_default();
            Ok(balance.checked_sub(amount)?)
        },
    )?;

    let recipient_addr = deps.api.addr_validate(&recipient)?;
    BALANCES.update(
        deps.storage,
        &recipient_addr,
        env.block.height,
        |balance: Option<Uint128>| -> StdResult<_> {
            let balance = balance.unwrap_or_default();
            Ok(balance.checked_add(amount)?)
        },
    )?;

    // Record transaction metadata
    let tx_metadata = TransactionMetadata {
        timestamp: env.block.time.seconds(),
        amount,
        signature: signature.clone(),
        nonce,
    };
    TX_METADATA.save(deps.storage, (&info.sender, nonce), &tx_metadata)?;

    Ok(Response::new()
        .add_attribute("action", "transfer")
        .add_attribute("from", info.sender)
        .add_attribute("to", recipient)
        .add_attribute("amount", amount)
        .add_attribute("nonce", nonce.to_string()))
}
