use multiversx_sc_scenario::imports::*;
use tro_staking::{
    errors::ERR_INVALID_PAYMENT_TOKEN, voting::DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS,
};

use crate::config::{
    LP_TOKEN_ID_1, LP_TOKEN_ID_2, LP_TOKEN_ID_3, OWNER_ADDRESS, SC_ADDRESS, TRO_TOKEN_ID,
    UNSUPPORTED_LP_TOKEN_ID, USER_ADDRESS,
};

use super::test_setup::{check_staked_amount, setup_world_with_contract};

#[test]
fn tro_stake_should_succeed() {
    let mut world = setup_world_with_contract();

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .stake()
        .payment(EsdtTokenPayment::new(
            TRO_TOKEN_ID.to_token_identifier(),
            0u64,
            BigUint::from(1000u64),
        ))
        .returns(ExpectStatus(0u64))
        .run();
}

#[test]
fn lp_stake_should_succeed() {
    let mut world = setup_world_with_contract();

    for token_id in [LP_TOKEN_ID_1, LP_TOKEN_ID_2, LP_TOKEN_ID_3] {
        world
            .tx()
            .from(USER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(tro_staking::proxy::TroStakingProxy)
            .stake()
            .payment(EsdtTokenPayment::new(
                token_id.to_token_identifier(),
                0u64,
                BigUint::from(1000u64),
            ))
            .returns(ExpectStatus(0u64))
            .run();
    }
}

#[test]
fn stake_many_should_succeed() {
    let mut world = setup_world_with_contract();

    let stake_amount = 1000u64;
    let lp_token_ids = [TRO_TOKEN_ID, LP_TOKEN_ID_1, LP_TOKEN_ID_2, LP_TOKEN_ID_3];

    let mut payments = MultiEsdtPayment::new();
    for token_id in lp_token_ids {
        payments.push(EsdtTokenPayment::new(
            token_id.to_token_identifier(),
            0u64,
            BigUint::from(stake_amount),
        ));
    }

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .stake()
        .multi_esdt(payments)
        .returns(ExpectStatus(0u64))
        .run();
}

#[test]
fn stake_with_active_proposal_should_succeed() {
    let mut world = setup_world_with_contract();

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
        .stake()
        .payment(EsdtTokenPayment::new(
            TRO_TOKEN_ID.to_token_identifier(),
            0u64,
            BigUint::from(1000u64),
        ))
        .returns(ExpectStatus(0u64))
        .run();
}

#[test]
fn stake_unsupported_token_should_fail() {
    let mut world = setup_world_with_contract();

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .stake()
        .payment(EsdtTokenPayment::new(
            UNSUPPORTED_LP_TOKEN_ID.to_token_identifier(),
            0u64,
            BigUint::from(1000u64),
        ))
        .returns(ExpectMessage(ERR_INVALID_PAYMENT_TOKEN))
        .run();
}

#[test]
fn stake_should_update_staked_amount() {
    let mut world = setup_world_with_contract();

    let stake_amount = 1000u64;
    let tokens = [TRO_TOKEN_ID, LP_TOKEN_ID_1, LP_TOKEN_ID_2, LP_TOKEN_ID_3];
    for token_id in tokens {
        for i in 0..10 {
            world
                .tx()
                .from(USER_ADDRESS)
                .to(SC_ADDRESS)
                .typed(tro_staking::proxy::TroStakingProxy)
                .stake()
                .payment(EsdtTokenPayment::new(
                    token_id.to_token_identifier(),
                    0u64,
                    BigUint::from(stake_amount),
                ))
                .returns(ExpectStatus(0u64))
                .run();

            check_staked_amount(&mut world, USER_ADDRESS, token_id, stake_amount * (i + 1));
        }
    }
}

#[test]
fn batch_staking_should_update_staked_amount() {
    let mut world = setup_world_with_contract();

    let stake_amount = 1000u64;
    let tokens = [TRO_TOKEN_ID, LP_TOKEN_ID_1, LP_TOKEN_ID_2, LP_TOKEN_ID_3];

    let mut payments = MultiEsdtPayment::new();
    for token_id in tokens {
        payments.push(EsdtTokenPayment::new(
            token_id.to_token_identifier(),
            0u64,
            BigUint::from(stake_amount),
        ));
    }

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .stake()
        .multi_esdt(payments)
        .returns(ExpectStatus(0u64))
        .run();

    for token_id in tokens {
        check_staked_amount(&mut world, USER_ADDRESS, token_id, stake_amount * 10);
    }
}
