use cosmwasm_std::{Deps, StdResult, Uint128};
use crate::state::BALANCES;

pub fn query_balance(deps: Deps, address: String) -> StdResult<BalanceResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let balance = BALANCES.may_load(deps.storage, &addr)?.unwrap_or_default();
    Ok(BalanceResponse { balance })
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub balance: Uint128,
}
