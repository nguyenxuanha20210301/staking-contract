use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,            // Address of the contract owner
    pub reward_rate: Uint128,   // Reward per second (in USDC)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserStake {
    pub amount: Uint128,        // Amount of OCH staked
    pub reward_debt: Uint128,   // Reward debt for the user
    pub last_update: u64,       // Timestamp of the last update
}

pub const CONFIG: Item<Config> = Item::new("config"); // Stores contract configuration
pub const TOTAL_STAKED: Item<Uint128> = Item::new("total_staked"); // Total OCH staked in the contract
pub const USER_STAKES: Map<&Addr, UserStake> = Map::new("user_stakes"); // Mapping of user addresses to their stakes

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardSnapshot {
    pub total_staked: Uint128,  // Total OCH staked at this snapshot
    pub reward_per_second: Uint128, // Reward rate in USDC at this snapshot
    pub timestamp: u64,         // Snapshot timestamp
}

// History of reward snapshots (optional)
pub const REWARD_SNAPSHOTS: Map<u64, RewardSnapshot> = Map::new("reward_snapshots");

/// Helper to calculate pending rewards for a user
pub fn calculate_pending_rewards(user: &UserStake, reward_rate: Uint128, current_time: u64) -> Uint128 {
    let duration = current_time - user.last_update; // Time since the last update
    let reward = user.amount.u128() * reward_rate.u128() * duration as u128 / 1_000_000_000u128; // Calculate reward
    Uint128::from(reward) - user.reward_debt
}
