use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Instantiate message: Used to initialize the staking contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub reward_rate: Uint128, // Reward per second in USDC
}

/// Execute messages: Actions that can be performed on the contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Stake a certain amount of OCH
    Stake { amount: Uint128 },

    /// Unstake a certain amount of OCH
    Unstake { amount: Uint128 },

    /// Update the reward rate (can only be called by the owner)
    UpdateRewardRate { reward_rate: Uint128 },

    /// Claim pending rewards (withdraw earned USDC rewards)
    ClaimRewards {},

    /// Withdraw rewards (only the contract owner can withdraw unused USDC rewards)
    WithdrawRewards { amount: Uint128 },
}

/// Query messages: For retrieving information from the contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Get the current configuration (reward rate, owner, etc.)
    Config {},

    /// Get staking details for a specific user
    UserStake { address: Addr },

    /// Get total OCH staked in the contract
    TotalStaked {},

    /// Calculate pending rewards for a specific user
    PendingRewards { address: Addr },

    /// Calculate the Annual Percentage Rate (APR) for a specific user
    Apr { address: Addr },
}

/// Response for Config query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: Addr,
    pub reward_rate: Uint128,
}

/// Response for UserStake query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserStakeResponse {
    pub amount: Uint128,       // Amount of OCH staked by the user
    pub reward_debt: Uint128,  // Reward debt for the user
    pub last_update: u64,      // Last update time
}

/// Response for TotalStaked query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TotalStakedResponse {
    pub total_staked: Uint128,
}

/// Response for PendingRewards query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PendingRewardsResponse {
    pub pending_rewards: Uint128,
}

/// Response for APR query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AprResponse {
    pub apr: Uint128, // Annual Percentage Rate as a percentage (e.g., 5000 = 50.00%)
}
