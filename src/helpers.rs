use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
// use cw_storage_plus::Item;

use crate::state::{Config, CONFIG, TOTAL_STAKED, USER_STAKES, UserStake};

/// Get the configuration of the contract
pub fn get_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}

/// Update the reward rate in the configuration
pub fn update_reward_rate(storage: &mut dyn Storage, reward_rate: Uint128) -> StdResult<()> {
    CONFIG.update(storage, |mut config| {
        config.reward_rate = reward_rate;
        Ok(config)
    }).map(|_| ())
}

/// Get the total OCH staked in the contract
pub fn get_total_staked(storage: &dyn Storage) -> StdResult<Uint128> {
    TOTAL_STAKED.load(storage)
}

/// Increase the total OCH staked
pub fn increase_total_staked(storage: &mut dyn Storage, amount: Uint128) -> StdResult<()> {
    TOTAL_STAKED.update(storage, |current| -> StdResult<Uint128> {
        Ok(current + amount)
    }).map(|_| ())
}

/// Decrease the total OCH staked
pub fn decrease_total_staked(storage: &mut dyn Storage, amount: Uint128) -> StdResult<()> {
    TOTAL_STAKED.update(storage, |current| -> StdResult<Uint128> {
        Ok(current - amount)
    }).map(|_| ())
}

/// Get the staking information for a specific user
pub fn get_user_stake(storage: &dyn Storage, user: &Addr) -> StdResult<UserStake> {
    USER_STAKES.load(storage, user)
}

/// Save or update the staking information for a specific user
pub fn save_user_stake(storage: &mut dyn Storage, user: &Addr, stake: UserStake) -> StdResult<()> {
    USER_STAKES.save(storage, user, &stake)
}

/// Remove a user's staking information
pub fn remove_user_stake(storage: &mut dyn Storage, user: &Addr) -> StdResult<()> {
    USER_STAKES.remove(storage, user);
    Ok(())
}

/// Calculate pending rewards for a specific user
pub fn calculate_pending_rewards(
    storage: &dyn Storage,
    user: &Addr,
    current_time: u64,
) -> StdResult<Uint128> {
    let user_stake = get_user_stake(storage, user)?;
    let config = get_config(storage)?;
    let duration = current_time - user_stake.last_update;

    // Reward = (staked amount) * (reward rate per second) * (time elapsed)
    let reward = user_stake.amount.u128() * config.reward_rate.u128() * duration as u128 / 1_000_000_000u128;
    let pending_reward = Uint128::from(reward) - user_stake.reward_debt;

    Ok(pending_reward)
}
