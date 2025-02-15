use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use crate::error::ContractError;
use crate::state::{TOKEN_CONFIG, RiskParameters};

pub fn execute_update_risk_parameters(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_params: RiskParameters,
) -> Result<Response, ContractError> {
    let config = TOKEN_CONFIG.load(deps.storage)?;
    
    // Only owner can update risk parameters
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    // Validate new parameters
    if new_params.max_tx_value.is_zero() || 
       new_params.daily_limit.is_zero() || 
       new_params.cooling_period == 0 {
        return Err(ContractError::InvalidRiskParameters {});
    }

    // Update parameters
    RISK_PARAMS.save(deps.storage, &new_params)?;

    Ok(Response::new()
        .add_attribute("action", "update_risk_parameters")
        .add_attribute("max_tx_value", new_params.max_tx_value)
        .add_attribute("daily_limit", new_params.daily_limit)
        .add_attribute("cooling_period", new_params.cooling_period.to_string()))
}

pub fn execute_pause(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    TOKEN_CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        if info.sender != config.owner {
            return Err(ContractError::Unauthorized {}.into());
        }
        config.paused = true;
        Ok(config)
    })?;

    Ok(Response::new().add_attribute("action", "pause"))
}

pub fn execute_unpause(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    TOKEN_CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        if info.sender != config.owner {
            return Err(ContractError::Unauthorized {}.into());
        }
        config.paused = false;
        Ok(config)
    })?;

    Ok(Response::new().add_attribute("action", "unpause"))
}
