use cosmwasm_std::{Deps, StdResult};
use crate::state::{VESTING_SCHEDULES, VestingSchedule};

pub fn query_vesting_schedules(deps: Deps, address: String) -> StdResult<VestingSchedulesResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let schedules = VESTING_SCHEDULES.may_load(deps.storage, &addr)?.unwrap_or_default();
    Ok(VestingSchedulesResponse { schedules })
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VestingSchedulesResponse {
    pub schedules: Vec<VestingSchedule>,
}
