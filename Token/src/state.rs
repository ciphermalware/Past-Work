use cosmwasm_std::{Addr, BlockInfo, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map, SnapshotMap, Strategy};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub owner: Addr,
    pub paused: bool,
    pub transfer_limit: Uint128,
    pub rate_limit_window: u64,
    pub version: String,
    pub security_module: Option<Binary>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransactionMetadata {
    pub timestamp: u64,
    pub amount: Uint128,
    pub signature: Binary,
    pub nonce: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RiskParameters {
    pub max_tx_value: Uint128,
    pub daily_limit: Uint128,
    pub min_holding_period: u64,
    pub max_accounts_per_block: u32,
    pub cooling_period: u64,
}

pub const TOKEN_CONFIG: Item<TokenConfig> = Item::new("token_config_v2");
pub const RISK_PARAMS: Item<RiskParameters> = Item::new("risk_parameters");
pub const BALANCES: SnapshotMap<&Addr, Uint128> = SnapshotMap::new(
    "balances_v2",
    "balances_checkpoints",
    "balances_changelog",
    Strategy::EveryBlock,
);
pub const TX_METADATA: Map<(&Addr, u64), TransactionMetadata> = Map::new("tx_metadata");
pub const NONCES: Map<&Addr, u64> = Map::new("nonces");
