use cosmwasm_std::{entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, ConfigResponse, UserStakeResponse, TotalStakedResponse, PendingRewardsResponse, AprResponse};
use crate::state::{Config, UserStake, CONFIG, TOTAL_STAKED, USER_STAKES};
use crate::helpers::{get_config, update_reward_rate, get_total_staked, increase_total_staked, decrease_total_staked, get_user_stake, save_user_stake, remove_user_stake, calculate_pending_rewards};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        owner: info.sender.clone(),
        reward_rate: msg.reward_rate,
    };

    CONFIG.save(deps.storage, &config)?;
    TOTAL_STAKED.save(deps.storage, &Uint128::zero())?;

    Ok(Response::new().add_attribute("method", "instantiate").add_attribute("owner", info.sender))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Stake { amount } => execute_stake(deps, env, info, amount),
        ExecuteMsg::Unstake { amount } => execute_unstake(deps, env, info, amount),
        ExecuteMsg::UpdateRewardRate { reward_rate } => execute_update_reward_rate(deps, info, reward_rate),
        ExecuteMsg::ClaimRewards {} => execute_claim_rewards(deps, env, info),
        ExecuteMsg::WithdrawRewards { amount } => execute_withdraw_rewards(deps, info, amount),
    }
}

fn execute_stake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> StdResult<Response> {
    let mut user_stake = USER_STAKES.may_load(deps.storage, &info.sender)?.unwrap_or(UserStake {
        amount: Uint128::zero(),
        reward_debt: Uint128::zero(),
        last_update: env.block.time.seconds(),
    });

    let pending = calculate_pending_rewards(deps.storage, &info.sender, env.block.time.seconds())?;
    user_stake.reward_debt += pending;
    user_stake.amount += amount;
    user_stake.last_update = env.block.time.seconds();

    save_user_stake(deps.storage, &info.sender, user_stake)?;
    increase_total_staked(deps.storage, amount)?;

    Ok(Response::new().add_attribute("action", "stake").add_attribute("amount", amount.to_string()))
}

fn execute_unstake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> StdResult<Response> {
    let mut user_stake = get_user_stake(deps.storage, &info.sender)?;

    if user_stake.amount < amount {
        return Err(StdError::generic_err("Insufficient stake"));
    }

    let pending = calculate_pending_rewards(deps.storage, &info.sender, env.block.time.seconds())?;
    user_stake.reward_debt += pending;
    user_stake.amount -= amount;
    user_stake.last_update = env.block.time.seconds();

    if user_stake.amount.is_zero() {
        remove_user_stake(deps.storage, &info.sender)?;
    } else {
        save_user_stake(deps.storage, &info.sender, user_stake)?;
    }

    decrease_total_staked(deps.storage, amount)?;

    Ok(Response::new().add_attribute("action", "unstake").add_attribute("amount", amount.to_string()))
}

fn execute_update_reward_rate(
    deps: DepsMut,
    info: MessageInfo,
    reward_rate: Uint128,
) -> StdResult<Response> {
    let config = get_config(deps.storage)?;

    if info.sender != config.owner {
        return Err(StdError::generic_err("Unauthorized"));
    }

    update_reward_rate(deps.storage, reward_rate)?;

    Ok(Response::new().add_attribute("action", "update_reward_rate").add_attribute("new_rate", reward_rate.to_string()))
}

fn execute_claim_rewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> StdResult<Response> {
    let mut user_stake = get_user_stake(deps.storage, &info.sender)?;
    let pending = calculate_pending_rewards(deps.storage, &info.sender, env.block.time.seconds())?;

    user_stake.reward_debt = Uint128::zero();
    user_stake.last_update = env.block.time.seconds();
    save_user_stake(deps.storage, &info.sender, user_stake)?;

    Ok(Response::new().add_attribute("action", "claim_rewards").add_attribute("rewards", pending.to_string()))
}

fn execute_withdraw_rewards(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> StdResult<Response> {
    let config = get_config(deps.storage)?;

    if info.sender != config.owner {
        return Err(StdError::generic_err("Unauthorized"));
    }

    Ok(Response::new().add_attribute("action", "withdraw_rewards").add_attribute("amount", amount.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::UserStake { address } => to_json_binary(&query_user_stake(deps, address)?),
        QueryMsg::TotalStaked {} => to_json_binary(&query_total_staked(deps)?),
        QueryMsg::PendingRewards { address } => to_json_binary(&query_pending_rewards(deps, env, address)?),
        QueryMsg::Apr { address } => to_json_binary(&query_apr(deps, address)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = get_config(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
        reward_rate: config.reward_rate,
    })
}

fn query_user_stake(deps: Deps, address: Addr) -> StdResult<UserStakeResponse> {
    let stake = get_user_stake(deps.storage, &address)?;
    Ok(UserStakeResponse {
        amount: stake.amount,
        reward_debt: stake.reward_debt,
        last_update: stake.last_update,
    })
}

fn query_total_staked(deps: Deps) -> StdResult<TotalStakedResponse> {
    let total_staked = get_total_staked(deps.storage)?;
    Ok(TotalStakedResponse { total_staked })
}

fn query_pending_rewards(deps: Deps, env: Env, address: Addr) -> StdResult<PendingRewardsResponse> {
    let pending = calculate_pending_rewards(deps.storage, &address, env.block.time.seconds())?;
    Ok(PendingRewardsResponse {
        pending_rewards: pending,
    })
}

fn query_apr(deps: Deps, address: Addr) -> StdResult<AprResponse> {
    let config = get_config(deps.storage)?;
    let total_staked = get_total_staked(deps.storage)?;

    if total_staked.is_zero() {
        return Ok(AprResponse { apr: Uint128::zero() });
    }

    let _user_stake = get_user_stake(deps.storage, &address)?;
    let apr = config.reward_rate * Uint128::from(12u128) * Uint128::from(100u128) / total_staked;

    Ok(AprResponse { apr })
}
