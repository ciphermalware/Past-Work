use cosmwasm_std::{Deps, StdResult, Uint128};
use crate::state::ALLOWANCES;

pub fn query_allowance(
    deps: Deps,
    owner: String,
    spender: String,
) -> StdResult<AllowanceResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let spender_addr = deps.api.addr_validate(&spender)?;
    
    let allowance = ALLOWANCES
        .may_load(deps.storage, (&owner_addr, &spender_addr))?
        .unwrap_or_default();
    
    Ok(AllowanceResponse { allowance })
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllowanceResponse {
    pub allowance: Uint128,
}
