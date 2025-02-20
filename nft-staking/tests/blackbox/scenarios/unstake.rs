use multiversx_sc::types::{EsdtTokenPayment, MultiValueManagedVec};
use multiversx_sc_scenario::{
    imports::SetStateStep, managed_biguint, ExpectError, ExpectStatus, ScenarioTxRun,
};
use nft_staking::constants::{
    DEFAULT_NFT_SCORE, ERR_NO_UNSTAKED_ITEMS, ERR_STAKING_DISABLED,
    ERR_USER_HAS_NOT_ENOUGH_STAKED_BALANCE, UNSTAKE_PENALTY,
};

use crate::{
    blackbox::{
        helpers::{
            check_aggregated_staking_score, check_user_staking_score, send_disable_staking_tx,
            send_disallow_collection_tx, send_stake_tx, send_unstake_tx,
        },
        test_setup::setup_world_with_contract,
    },
    config::*,
};

#[test]
fn should_not_be_able_to_unstake_if_nothing_staked() {
    let mut world = setup_world_with_contract();

    let unstake_request = MultiValueManagedVec::from_single_item(EsdtTokenPayment::new(
        NFT_TOKEN_ID.to_token_identifier(),
        1,
        managed_biguint!(1),
    ));

    world
        .tx()
        .from(USER_ADDRESS.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .unstake(unstake_request)
        .returns(ExpectError(4u64, ERR_USER_HAS_NOT_ENOUGH_STAKED_BALANCE))
        .run();
}

#[test]
fn should_not_be_able_to_unstake_if_not_enough_staked_balance() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(SFT_TOKEN_ID, 1, 1)]);

    let unstake_request = MultiValueManagedVec::from_single_item(EsdtTokenPayment::new(
        SFT_TOKEN_ID.to_token_identifier(),
        1,
        managed_biguint!(2),
    ));
    world
        .tx()
        .from(USER_ADDRESS.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .unstake(unstake_request)
        .returns(ExpectError(4u64, ERR_USER_HAS_NOT_ENOUGH_STAKED_BALANCE))
        .run();
}

#[test]
fn should_not_be_able_to_unstake_other_user_assets() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &OWNER_ADDRESS, &[&(SFT_TOKEN_ID, 1, 2)]);
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(SFT_TOKEN_ID, 1, 1)]);

    let unstake_request = MultiValueManagedVec::from_single_item(EsdtTokenPayment::new(
        SFT_TOKEN_ID.to_token_identifier(),
        1,
        managed_biguint!(2),
    ));
    world
        .tx()
        .from(USER_ADDRESS.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .unstake(unstake_request)
        .returns(ExpectError(4u64, ERR_USER_HAS_NOT_ENOUGH_STAKED_BALANCE))
        .run();
}

#[test]
fn should_be_able_to_unstake_assets() {
    let mut world = setup_world_with_contract();

    send_stake_tx(
        &mut world,
        &USER_ADDRESS,
        &[
            &(NFT_TOKEN_ID, 1, 1),
            &(SFT_TOKEN_ID, 1, INITIAL_SFT_BALANCE),
        ],
    );
    send_unstake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);
    send_unstake_tx(
        &mut world,
        &USER_ADDRESS,
        &[&(SFT_TOKEN_ID, 1, INITIAL_SFT_BALANCE / 2)],
    );
    send_unstake_tx(
        &mut world,
        &USER_ADDRESS,
        &[&(SFT_TOKEN_ID, 1, INITIAL_SFT_BALANCE / 2)],
    );

    send_stake_tx(
        &mut world,
        &USER_ADDRESS,
        &[&(NFT_TOKEN_ID, 2, 1), &(SFT_TOKEN_ID, 2, 1)],
    );
    send_unstake_tx(
        &mut world,
        &USER_ADDRESS,
        &[&(NFT_TOKEN_ID, 2, 1), &(SFT_TOKEN_ID, 2, 1)],
    );
}

#[test]
fn should_not_be_able_to_unstake_if_staking_is_disabled() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);
    send_disable_staking_tx(&mut world);

    let unstake_request = MultiValueManagedVec::from_single_item(EsdtTokenPayment::new(
        SFT_TOKEN_ID.to_token_identifier(),
        1,
        managed_biguint!(2),
    ));
    world
        .tx()
        .from(USER_ADDRESS.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .unstake(unstake_request)
        .returns(ExpectError(4u64, ERR_STAKING_DISABLED))
        .run();
}

#[test]
fn should_allow_unstake_of_previously_supported_already_staked_collection() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);

    send_disallow_collection_tx(&mut world, &NFT_TOKEN_ID);

    send_unstake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);
}

#[test]
fn should_not_be_able_to_claim_before_penalty_is_over() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);
    send_unstake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);

    // immediately after unstake
    world
        .tx()
        .from(USER_ADDRESS.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .claim_unstaked()
        .returns(ExpectError(4u64, ERR_NO_UNSTAKED_ITEMS))
        .run();

    world.set_state_step(SetStateStep::new().block_timestamp(UNSTAKE_PENALTY - 1));

    // before penalty is over
    world
        .tx()
        .from(USER_ADDRESS.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .claim_unstaked()
        .returns(ExpectError(4u64, ERR_NO_UNSTAKED_ITEMS))
        .run();
}

#[test]
fn should_be_able_to_claim_after_penalty_is_over() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);
    send_unstake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);

    world.set_state_step(SetStateStep::new().block_timestamp(UNSTAKE_PENALTY));

    world
        .tx()
        .from(USER_ADDRESS.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .claim_unstaked()
        .returns(ExpectStatus(0u64))
        .run();

    // TODO: add account balance check after claim unstaked to enforce "send" calls
}

#[test]
fn unstaking_should_decrease_staking_scores() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);
    send_unstake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);

    check_aggregated_staking_score(&mut world, 0);
    check_user_staking_score(&mut world, &USER_ADDRESS, 0);
}

#[test]
fn unstaking_multiple_assets_should_decrease_staking_scores() {
    let mut world = setup_world_with_contract();

    send_stake_tx(
        &mut world,
        &USER_ADDRESS,
        &[
            &(NFT_TOKEN_ID, 1, 1),                   // 1 point
            &(SFT_TOKEN_ID, 1, 1),                   // 1 point
            &(SFT_TOKEN_ID, 1, 1),                   // 1 point
            &(SFT_TOKEN_ID, 2, INITIAL_SFT_BALANCE), // 10 points
        ],
    );
    send_unstake_tx(
        &mut world,
        &USER_ADDRESS,
        &[
            &(NFT_TOKEN_ID, 1, 1),                   // 1 point
            &(SFT_TOKEN_ID, 1, 1),                   // 1 point
            &(SFT_TOKEN_ID, 1, 1),                   // 1 point
            &(SFT_TOKEN_ID, 2, INITIAL_SFT_BALANCE), // 10 points
        ],
    );

    check_aggregated_staking_score(&mut world, 0);
    check_user_staking_score(&mut world, &USER_ADDRESS, 0);
}

#[test]
fn unstaking_should_only_decrease_staking_scores_for_the_unstaked_assets() {
    let mut world = setup_world_with_contract();

    send_stake_tx(
        &mut world,
        &USER_ADDRESS,
        &[
            &(NFT_TOKEN_ID, 1, 1),                   // 1 point
            &(SFT_TOKEN_ID, 1, 1),                   // 1 point
            &(SFT_TOKEN_ID, 1, 1),                   // 1 point
            &(SFT_TOKEN_ID, 2, INITIAL_SFT_BALANCE), // 10 points
        ],
    );
    send_unstake_tx(
        &mut world,
        &USER_ADDRESS,
        &[
            &(NFT_TOKEN_ID, 1, 1), // 1 point
            &(SFT_TOKEN_ID, 1, 1), // 1 point
            &(SFT_TOKEN_ID, 1, 1), // 1 point
        ],
    );

    check_aggregated_staking_score(&mut world, INITIAL_SFT_BALANCE * DEFAULT_NFT_SCORE);
    check_user_staking_score(
        &mut world,
        &USER_ADDRESS,
        INITIAL_SFT_BALANCE * DEFAULT_NFT_SCORE,
    );
}

#[test]
fn double_sft_unstake_fails_if_not_enough_staked_balance() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(SFT_TOKEN_ID, 1, 1)]);
    send_stake_tx(&mut world, &OWNER_ADDRESS, &[&(SFT_TOKEN_ID, 1, 1)]);

    let mut unstake_request = MultiValueManagedVec::new();
    unstake_request.push(EsdtTokenPayment::new(
        SFT_TOKEN_ID.to_token_identifier(),
        1,
        managed_biguint!(1),
    ));
    unstake_request.push(EsdtTokenPayment::new(
        SFT_TOKEN_ID.to_token_identifier(),
        1,
        managed_biguint!(1),
    ));

    world
        .tx()
        .from(USER_ADDRESS.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .unstake(unstake_request)
        .returns(ExpectError(4u64, ERR_USER_HAS_NOT_ENOUGH_STAKED_BALANCE))
        .run();
}
