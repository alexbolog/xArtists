use multiversx_sc_scenario::imports::*;
use tro_staking::errors::ERR_INVALID_PAYMENT_TOKEN;

use crate::config::{
    LP_TOKEN_ID_1, LP_TOKEN_ID_2, LP_TOKEN_ID_3, SC_ADDRESS, TRO_TOKEN_ID, UNSUPPORTED_LP_TOKEN_ID,
    USER_ADDRESS,
};

use super::test_setup::setup_world_with_contract;

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
