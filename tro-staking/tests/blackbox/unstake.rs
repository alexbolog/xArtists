use multiversx_sc_scenario::imports::*;
use tro_staking::{
    errors::{ERR_INSUFFICIENT_STAKE, ERR_PROPOSAL_ACTIVE},
    voting::DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS,
};

use crate::config::*;

use super::test_setup::{check_staked_amount, setup_world_with_contract};

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

#[test]
fn unstake_should_fail_if_proposal_is_active() {
    let mut world = setup_world_with_contract();

    stake_token(&mut world, TRO_TOKEN_ID, 1000);

    let mut unstake_args = MultiValueEncoded::new();
    unstake_args.push(MultiValue2((
        TRO_TOKEN_ID.to_token_identifier(),
        BigUint::from(1000u64),
    )));

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .create_proposal(
            ManagedBuffer::new_from_bytes(b"title"),
            ManagedBuffer::new_from_bytes(b"description"),
            BigUint::from(1u64),
            OptionalValue::<u64>::None,
            OptionalValue::<u64>::None,
            MultiValueEncoded::new(),
        )
        .returns(ExpectStatus(0u64))
        .run();

    world.set_state_step(
        SetStateStep::new().block_timestamp(DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS + 1),
    );

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .unstake(unstake_args)
        .returns(ExpectMessage(ERR_PROPOSAL_ACTIVE))
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

#[test]
fn unstake_should_update_staked_amount() {
    let mut world = setup_world_with_contract();

    let token_ids = [TRO_TOKEN_ID, LP_TOKEN_ID_1, LP_TOKEN_ID_2, LP_TOKEN_ID_3];

    for token_id in token_ids {
        stake_token(&mut world, token_id, 1000);
    }

    for token_id in token_ids {
        let mut unstake_args = MultiValueEncoded::new();
        unstake_args.push(MultiValue2((
            token_id.to_token_identifier(),
            BigUint::from(1000u64),
        )));

        world
            .tx()
            .from(USER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(tro_staking::proxy::TroStakingProxy)
            .unstake(unstake_args)
            .returns(ExpectStatus(0u64))
            .run();

        check_staked_amount(&mut world, USER_ADDRESS, token_id, 0);

        stake_token(&mut world, token_id, 1000);

        let mut unstake_args = MultiValueEncoded::new();
        unstake_args.push(MultiValue2((
            token_id.to_token_identifier(),
            BigUint::from(500u64),
        )));

        world
            .tx()
            .from(USER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(tro_staking::proxy::TroStakingProxy)
            .unstake(unstake_args)
            .returns(ExpectStatus(0u64))
            .run();

        check_staked_amount(&mut world, USER_ADDRESS, token_id, 500);
    }
}
