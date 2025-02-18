use multiversx_sc_scenario::{imports::SetStateStep, rust_biguint};
use nft_staking::{constants::DEFAULT_NFT_SCORE, reward::reward_rate::REWARD_RATE_DENOMINATION};

use crate::{
    blackbox::{
        helpers::{
            check_if_token_is_reward_token, check_last_distribution_round, check_reward_rate,
            send_distribute_rewards_tx, send_stake_tx,
        },
        test_setup::setup_world_with_contract,
    },
    config::{NFT_TOKEN_ID, REWARD_TOKEN_ID_1, REWARD_TOKEN_ID_2, USER_ADDRESS},
};

#[test]
fn manual_reward_distribution_correctly_updates_reward_rate() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 2, 1)]);

    let amount_to_distribute = DEFAULT_NFT_SCORE;
    let expected_reward_rate = rust_biguint!(REWARD_RATE_DENOMINATION / 2);

    send_distribute_rewards_tx(&mut world, REWARD_TOKEN_ID_1, amount_to_distribute);

    check_reward_rate(&mut world, &REWARD_TOKEN_ID_1, expected_reward_rate);
}

#[test]
fn manual_reward_distribution_does_not_update_last_distribution_round() {
    let mut world = setup_world_with_contract();

    send_distribute_rewards_tx(&mut world, REWARD_TOKEN_ID_1, DEFAULT_NFT_SCORE);
    check_last_distribution_round(&mut world, 0);
}

#[test]
fn manual_reward_distribution_does_not_fail_with_nothing_staked() {
    let mut world = setup_world_with_contract();
    world.set_state_step(SetStateStep::new().block_round(100));

    send_distribute_rewards_tx(&mut world, REWARD_TOKEN_ID_1, DEFAULT_NFT_SCORE);

    check_last_distribution_round(&mut world, 0);
}

#[test]
fn manual_reward_distribution_adds_new_token_to_reward_token_list() {
    let mut world = setup_world_with_contract();

    send_distribute_rewards_tx(&mut world, REWARD_TOKEN_ID_1, DEFAULT_NFT_SCORE);
    send_distribute_rewards_tx(&mut world, REWARD_TOKEN_ID_2, DEFAULT_NFT_SCORE);

    check_if_token_is_reward_token(&mut world, &REWARD_TOKEN_ID_1);
    check_if_token_is_reward_token(&mut world, &REWARD_TOKEN_ID_2);
}
