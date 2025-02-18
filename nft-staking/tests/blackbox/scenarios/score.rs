use nft_staking::constants::DEFAULT_NFT_SCORE;

use crate::{
    blackbox::{
        helpers::{
            check_aggregated_staking_score, send_set_collection_nonce_score_tx,
            send_set_collection_score_tx, send_stake_tx, send_unstake_tx,
        },
        test_setup::setup_world_with_contract,
    },
    config::{NFT_TOKEN_ID, SFT_TOKEN_ID, USER_ADDRESS},
};

#[test]
fn collection_score_should_override_default_score() {
    let mut world = setup_world_with_contract();
    send_set_collection_score_tx(&mut world, &NFT_TOKEN_ID, DEFAULT_NFT_SCORE * 2);

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);

    check_aggregated_staking_score(&mut world, DEFAULT_NFT_SCORE * 2);
}

#[test]
fn nonce_score_should_override_default_score() {
    let mut world = setup_world_with_contract();
    send_set_collection_nonce_score_tx(&mut world, &NFT_TOKEN_ID, 1, DEFAULT_NFT_SCORE * 2);

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);

    check_aggregated_staking_score(&mut world, DEFAULT_NFT_SCORE * 2);
}

#[test]
fn nonce_score_should_override_collection_score() {
    let mut world = setup_world_with_contract();
    send_set_collection_score_tx(&mut world, &NFT_TOKEN_ID, DEFAULT_NFT_SCORE * 2);
    send_set_collection_nonce_score_tx(&mut world, &NFT_TOKEN_ID, 1, DEFAULT_NFT_SCORE * 3);

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);

    check_aggregated_staking_score(&mut world, DEFAULT_NFT_SCORE * 3);
}

#[test]
fn quantity_should_be_accounted_for_correctly() {
    let mut world = setup_world_with_contract();
    send_set_collection_nonce_score_tx(&mut world, &SFT_TOKEN_ID, 1, DEFAULT_NFT_SCORE * 3);
    send_set_collection_score_tx(&mut world, &SFT_TOKEN_ID, DEFAULT_NFT_SCORE * 2);

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(SFT_TOKEN_ID, 1, 2)]); // 3 points * 2 = 6
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(SFT_TOKEN_ID, 2, 1)]); // 2 points * 1 = 2
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]); // 1 point * 1 = 1

    let total_score = 9 * DEFAULT_NFT_SCORE;
    check_aggregated_staking_score(&mut world, total_score);
}

#[test]
fn staking_different_score_assets_should_be_accounted_for_correctly() {
    let mut world = setup_world_with_contract();
    send_set_collection_score_tx(&mut world, &NFT_TOKEN_ID, DEFAULT_NFT_SCORE * 2);
    send_set_collection_nonce_score_tx(&mut world, &NFT_TOKEN_ID, 1, DEFAULT_NFT_SCORE * 3);

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]); // 3 points, nonce 1
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 2, 1)]); // 2 points, collection score
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(SFT_TOKEN_ID, 1, 1)]); // 1 point, default score

    let total_score = 6 * DEFAULT_NFT_SCORE;

    check_aggregated_staking_score(&mut world, total_score);
}

#[test]
fn unstaking_different_score_assets_should_be_accounted_for_correctly() {
    let mut world = setup_world_with_contract();
    send_set_collection_score_tx(&mut world, &NFT_TOKEN_ID, DEFAULT_NFT_SCORE * 2);
    send_set_collection_nonce_score_tx(&mut world, &NFT_TOKEN_ID, 1, DEFAULT_NFT_SCORE * 3);

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 2, 1)]);
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(SFT_TOKEN_ID, 1, 1)]);

    send_unstake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);
    send_unstake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 2, 1)]);
    send_unstake_tx(&mut world, &USER_ADDRESS, &[&(SFT_TOKEN_ID, 1, 1)]);

    check_aggregated_staking_score(&mut world, 0);
}
