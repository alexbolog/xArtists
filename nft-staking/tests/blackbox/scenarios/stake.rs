use multiversx_sc::types::EsdtTokenPayment;
use multiversx_sc_scenario::{managed_biguint, ExpectError, ScenarioTxRun};
use nft_staking::constants::{
    DEFAULT_NFT_SCORE, ERR_NFT_COLLECTION_NOT_ALLOWED, ERR_STAKING_DISABLED,
};

use crate::{
    blackbox::{
        helpers::{
            check_aggregated_staking_score, check_staked_amount, check_user_staking_score,
            send_disable_staking_tx, send_stake_tx,
        },
        test_setup::setup_world_with_contract,
    },
    config::*,
};

#[test]
fn staking_allowed_nft_should_succeed() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);
}

#[test]
fn staking_single_allowed_sft_should_succeed() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(SFT_TOKEN_ID, 1, 1)]);
}

#[test]
fn staking_multiple_allowed_assets_should_succeed() {
    let mut world = setup_world_with_contract();

    send_stake_tx(
        &mut world,
        &USER_ADDRESS,
        &[
            &(NFT_TOKEN_ID, 1, 1),
            &(SFT_TOKEN_ID, 1, 1),
            &(SFT_TOKEN_ID, 2, 1),
        ],
    );
}

#[test]
fn staking_multiple_allowed_sfts_with_same_token_id_and_nonce_should_succeed() {
    let mut world = setup_world_with_contract();

    send_stake_tx(
        &mut world,
        &USER_ADDRESS,
        &[&(SFT_TOKEN_ID, 1, 1), &(SFT_TOKEN_ID, 1, 1)],
    );
}

#[test]
fn successful_staking_should_increase_staked_amount() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(SFT_TOKEN_ID, 1, 1)]);

    check_staked_amount(&mut world, &USER_ADDRESS, &NFT_TOKEN_ID, 1, 1);
    check_staked_amount(&mut world, &USER_ADDRESS, &SFT_TOKEN_ID, 1, 1);
}

#[test]
fn successful_staking_should_increase_staked_amount_even_if_sfts_are_sent_twice_in_same_tx() {
    let mut world = setup_world_with_contract();

    send_stake_tx(
        &mut world,
        &USER_ADDRESS,
        &[&(SFT_TOKEN_ID, 1, 1), &(SFT_TOKEN_ID, 1, 1)],
    );

    check_staked_amount(&mut world, &USER_ADDRESS, &SFT_TOKEN_ID, 1, 2);
}

#[test]
fn staking_unsupported_asset_should_fail() {
    let mut world = setup_world_with_contract();

    world
        .tx()
        .from(USER_ADDRESS.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .stake()
        .with_esdt_transfer(EsdtTokenPayment::new(
            UNSUPPORTED_NFT_TOKEN_ID.to_token_identifier(),
            1,
            managed_biguint!(1),
        ))
        .returns(ExpectError(4u64, ERR_NFT_COLLECTION_NOT_ALLOWED))
        .run();
}

#[test]
fn should_not_be_able_to_stake_if_staking_is_disabled() {
    let mut world = setup_world_with_contract();

    send_disable_staking_tx(&mut world);

    world
        .tx()
        .from(USER_ADDRESS.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .stake()
        .with_esdt_transfer(EsdtTokenPayment::new(
            UNSUPPORTED_NFT_TOKEN_ID.to_token_identifier(),
            1,
            managed_biguint!(1),
        ))
        .returns(ExpectError(4u64, ERR_STAKING_DISABLED))
        .run();
}

#[test]
fn stake_should_increase_staking_scores() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);

    check_aggregated_staking_score(&mut world, DEFAULT_NFT_SCORE);
    check_user_staking_score(&mut world, &USER_ADDRESS, DEFAULT_NFT_SCORE);
}

#[test]
fn staking_multiple_assets_should_increase_staking_scores() {
    let mut world = setup_world_with_contract();

    send_stake_tx(
        &mut world,
        &USER_ADDRESS,
        &[
            &(NFT_TOKEN_ID, 1, 1), // 1 point
            &(SFT_TOKEN_ID, 1, 1), // 1 point
            &(SFT_TOKEN_ID, 1, 1), // 1 point
            &(SFT_TOKEN_ID, 2, INITIAL_SFT_BALANCE), // 10 points
        ],
    );

    let expected_score = 13; // 1 + 1 + 1 + 10
    check_aggregated_staking_score(&mut world, expected_score);
    check_user_staking_score(&mut world, &USER_ADDRESS, expected_score);
}

#[test]
fn increasing_stake_should_increase_staking_scores() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);

    check_aggregated_staking_score(&mut world, DEFAULT_NFT_SCORE);
    check_user_staking_score(&mut world, &USER_ADDRESS, DEFAULT_NFT_SCORE);

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 2, 1)]);

    check_aggregated_staking_score(&mut world, DEFAULT_NFT_SCORE * 2);
    check_user_staking_score(&mut world, &USER_ADDRESS, DEFAULT_NFT_SCORE * 2);
}
