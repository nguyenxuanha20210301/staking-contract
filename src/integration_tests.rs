use cosmwasm_std::{coin, coins, Addr, Uint128};
use cosmwasm_vm::testing::{instantiate, execute, query};
use cosmwasm_vm::testing::{mock_dependencies, mock_env, mock_info};
use staking_contract::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, ConfigResponse, UserStakeResponse, TotalStakedResponse, PendingRewardsResponse};

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        reward_rate: Uint128::new(100),
    };
    let info = mock_info("creator", &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(res.messages.len(), 0);

    let config: ConfigResponse = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    assert_eq!(config.owner, Addr::unchecked("creator"));
    assert_eq!(config.reward_rate, Uint128::new(100));
}

#[test]
fn stake_tokens() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        reward_rate: Uint128::new(100),
    };
    let info = mock_info("creator", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let stake_msg = ExecuteMsg::Stake {
        amount: Uint128::new(1000),
    };
    let info = mock_info("user1", &[]);
    execute(deps.as_mut(), mock_env(), info, stake_msg).unwrap();

    let user_stake: UserStakeResponse = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::UserStake {
            address: Addr::unchecked("user1"),
        },
    )
    .unwrap();
    assert_eq!(user_stake.amount, Uint128::new(1000));
}

#[test]
fn unstake_tokens() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        reward_rate: Uint128::new(100),
    };
    let info = mock_info("creator", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let stake_msg = ExecuteMsg::Stake {
        amount: Uint128::new(1000),
    };
    let info = mock_info("user1", &[]);
    execute(deps.as_mut(), mock_env(), info, stake_msg).unwrap();

    let unstake_msg = ExecuteMsg::Unstake {
        amount: Uint128::new(500),
    };
    execute(deps.as_mut(), mock_env(), info, unstake_msg).unwrap();

    let user_stake: UserStakeResponse = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::UserStake {
            address: Addr::unchecked("user1"),
        },
    )
    .unwrap();
    assert_eq!(user_stake.amount, Uint128::new(500));
}

#[test]
fn claim_rewards() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        reward_rate: Uint128::new(10),
    };
    let info = mock_info("creator", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let stake_msg = ExecuteMsg::Stake {
        amount: Uint128::new(1000),
    };
    let info = mock_info("user1", &[]);
    execute(deps.as_mut(), mock_env(), info, stake_msg).unwrap();

    // Simulate time passing
    let mut env = mock_env();
    env.block.time = env.block.time.plus_seconds(10);

    let claim_msg = ExecuteMsg::ClaimRewards {};
    execute(deps.as_mut(), env.clone(), info.clone(), claim_msg).unwrap();

    let pending_rewards: PendingRewardsResponse = query(
        deps.as_ref(),
        env,
        QueryMsg::PendingRewards {
            address: Addr::unchecked("user1"),
        },
    )
    .unwrap();
    assert_eq!(pending_rewards.pending_rewards, Uint128::zero());
}
