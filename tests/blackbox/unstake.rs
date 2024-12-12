use multiversx_sc_scenario::imports::*;
use tro_staking::errors::ERR_INSUFFICIENT_STAKE;

use crate::config::*;

use super::test_setup::setup_world_with_contract;

#[test]
fn unstake_tro_should_succeed() {
    let mut world = setup_world_with_contract();
    let stake_amount = 1000;

    stake_token(&mut world, TRO_TOKEN_ID, stake_amount);

    let mut unstake_args = MultiValueEncoded::new();
    unstake_args.push(MultiValue2((
        TRO_TOKEN_ID.to_token_identifier(),
        BigUint::from(stake_amount),
    )));

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .unstake(unstake_args)
        .returns(ExpectStatus(0u64))
        .run();
}

#[test]
fn unstake_lp_should_succeed() {
    let mut world = setup_world_with_contract();
    let stake_amount = 1000;

    let lp_token_ids = [LP_TOKEN_ID_1, LP_TOKEN_ID_2, LP_TOKEN_ID_3];
    for token_id in lp_token_ids {
        stake_token(&mut world, token_id, stake_amount);

        let mut unstake_args = MultiValueEncoded::new();
        unstake_args.push(MultiValue2((
            token_id.to_token_identifier(),
            BigUint::from(stake_amount),
        )));

        world
            .tx()
            .from(USER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(tro_staking::proxy::TroStakingProxy)
            .unstake(unstake_args)
            .returns(ExpectStatus(0u64))
            .run();
    }
}

#[test]
fn partial_unstake_should_succeed() {
    let mut world = setup_world_with_contract();
    let stake_amount = 1000;
    let unstake_amount = stake_amount / 2;

    stake_token(&mut world, TRO_TOKEN_ID, stake_amount);

    let mut unstake_args = MultiValueEncoded::new();
    unstake_args.push(MultiValue2((
        TRO_TOKEN_ID.to_token_identifier(),
        BigUint::from(unstake_amount),
    )));

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .unstake(unstake_args)
        .returns(ExpectStatus(0u64))
        .run();
}

#[test]
fn unstake_total_amount_batched_should_succeed() {
    let mut world = setup_world_with_contract();
    let stake_amount = 1000u64;

    let lp_token_ids = [LP_TOKEN_ID_1, LP_TOKEN_ID_2, LP_TOKEN_ID_3];
    for token_id in lp_token_ids {
        stake_token(&mut world, token_id, stake_amount);
    }

    let unstake_amount_per_batch = stake_amount / 4;

    for token_id in lp_token_ids {
        let mut unstake_args = MultiValueEncoded::new();
        unstake_args.push(MultiValue2((
            token_id.to_token_identifier(),
            BigUint::from(unstake_amount_per_batch),
        )));

        world
            .tx()
            .from(USER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(tro_staking::proxy::TroStakingProxy)
            .unstake(unstake_args)
            .returns(ExpectStatus(0u64))
            .run();
    }
}

#[test]
fn unstaking_same_token_multiple_times_under_total_staked_should_succeed() {
    let mut world = setup_world_with_contract();
    let stake_amount = 1000u64;
    let unstake_amount = stake_amount / 2;

    stake_token(&mut world, TRO_TOKEN_ID, stake_amount);

    let mut unstake_args = MultiValueEncoded::new();
    unstake_args.push(MultiValue2((
        TRO_TOKEN_ID.to_token_identifier(),
        BigUint::from(unstake_amount),
    )));
    unstake_args.push(MultiValue2((
        TRO_TOKEN_ID.to_token_identifier(),
        BigUint::from(unstake_amount),
    )));

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .unstake(unstake_args)
        .returns(ExpectStatus(0u64))
        .run();
}

#[test]
fn multiple_unstake_single_tx_should_succeed() {
    let mut world = setup_world_with_contract();
    let stake_amount = 1000;

    let lp_token_ids = [LP_TOKEN_ID_1, LP_TOKEN_ID_2, LP_TOKEN_ID_3];
    for token_id in lp_token_ids {
        stake_token(&mut world, token_id, stake_amount);
    }

    let mut unstake_args = MultiValueEncoded::new();
    for token_id in lp_token_ids {
        unstake_args.push(MultiValue2((
            token_id.to_token_identifier(),
            BigUint::from(stake_amount),
        )));
    }

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .unstake(unstake_args)
        .returns(ExpectStatus(0u64))
        .run();
}

#[test]
fn unstake_more_than_staked_should_fail_for_any_token() {
    let mut world = setup_world_with_contract();
    let stake_amount = 1000;
    let unstake_amount = stake_amount + 1;

    let lp_token_ids = [TRO_TOKEN_ID, LP_TOKEN_ID_1, LP_TOKEN_ID_2, LP_TOKEN_ID_3];
    for token_id in lp_token_ids {
        stake_token(&mut world, token_id, stake_amount);

        let mut unstake_args = MultiValueEncoded::new();
        unstake_args.push(MultiValue2((
            token_id.to_token_identifier(),
            BigUint::from(unstake_amount),
        )));

        world
            .tx()
            .from(USER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(tro_staking::proxy::TroStakingProxy)
            .unstake(unstake_args)
            .returns(ExpectMessage(ERR_INSUFFICIENT_STAKE))
            .run();
    }
}

#[test]
fn unstake_with_nothing_staked_should_fail() {
    let mut world = setup_world_with_contract();

    let mut unstake_args = MultiValueEncoded::new();
    unstake_args.push(MultiValue2((
        TRO_TOKEN_ID.to_token_identifier(),
        BigUint::from(1u64),
    )));

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .unstake(unstake_args)
        .returns(ExpectMessage(ERR_INSUFFICIENT_STAKE))
        .run();
}

#[test]
fn unstaking_same_token_multiple_times_over_total_staked_should_fail() {
    let mut world = setup_world_with_contract();
    let stake_amount = 1000u64;
    let unstake_amount = stake_amount + 1;

    stake_token(&mut world, TRO_TOKEN_ID, stake_amount);

    let mut unstake_args = MultiValueEncoded::new();
    unstake_args.push(MultiValue2((
        TRO_TOKEN_ID.to_token_identifier(),
        BigUint::from(unstake_amount),
    )));
    unstake_args.push(MultiValue2((
        TRO_TOKEN_ID.to_token_identifier(),
        BigUint::from(unstake_amount),
    )));
    unstake_args.push(MultiValue2((
        TRO_TOKEN_ID.to_token_identifier(),
        BigUint::from(unstake_amount),
    )));

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .unstake(unstake_args)
        .returns(ExpectMessage(ERR_INSUFFICIENT_STAKE))
        .run();
}

fn stake_token(world: &mut ScenarioWorld, token_id: TestTokenIdentifier, amount: u64) {
    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .stake()
        .payment(EsdtTokenPayment::new(
            token_id.to_token_identifier(),
            0u64,
            BigUint::from(amount),
        ))
        .returns(ExpectStatus(0u64))
        .run();
}
