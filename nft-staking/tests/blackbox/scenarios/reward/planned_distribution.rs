use multiversx_sc_scenario::imports::SetStateStep;
use nft_staking::{constants::DEFAULT_NFT_SCORE, reward::reward_rate::REWARD_RATE_DENOMINATION};

use crate::{
    blackbox::{
        helpers::{
            check_distribution_amount_per_round, check_if_token_is_reward_token,
            check_last_distribution_round, check_reward_rate, send_set_distribution_plan_tx,
            send_stake_tx,
        },
        test_setup::setup_world_with_contract,
    },
    config::{NFT_TOKEN_ID, REWARD_TOKEN_ID_1, USER_ADDRESS},
};

#[test]
fn planned_reward_distribution_round_distribution_amount_calculation_should_work() {
    let mut world = setup_world_with_contract();

    check_distribution_amount_per_round(
        &mut world,
        0,
        10,
        REWARD_RATE_DENOMINATION,
        REWARD_RATE_DENOMINATION / 10,
    );
}

#[test]
fn planned_reward_distribution_adds_new_token_to_reward_token_list() {
    let mut world = setup_world_with_contract();

    send_set_distribution_plan_tx(&mut world, REWARD_TOKEN_ID_1, 0, 100, 100); // 1 tokens per round

    check_if_token_is_reward_token(&mut world, &REWARD_TOKEN_ID_1);
}

#[test]
fn executed_planned_distribution_should_update_last_distribution_round() {
    let mut world = setup_world_with_contract();

    send_set_distribution_plan_tx(&mut world, REWARD_TOKEN_ID_1, 0, 100, 100); // 1 tokens per round
    world.set_state_step(SetStateStep::new().block_round(1)); // start at round 1
    check_last_distribution_round(&mut world, 0);

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]); // triggers distribution
    check_last_distribution_round(&mut world, 1);

    world.set_state_step(SetStateStep::new().block_round(2)); // advance to next round

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 2, 1)]); // trigger distribution
    check_last_distribution_round(&mut world, 2);
}

#[test]
fn planned_distribution_happens_exactly_once() {
    let mut world = setup_world_with_contract();

    send_set_distribution_plan_tx(&mut world, REWARD_TOKEN_ID_1, 0, 100, 100); // 1 tokens per round
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]); // single staker, takes entire reward pool, triggers distribution

    world.set_state_step(SetStateStep::new().block_round(1)); // advance to next round

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 2, 1)]); // trigger distribution

    // at this point, the state is:
    // - 2 tokens distributed across 2 rounds (round 0 and round 1)
    // - total staked score is 2, and owned by the same single staker

    let expected_reward_rate = REWARD_RATE_DENOMINATION; // 1 token per round
    check_reward_rate(&mut world, &REWARD_TOKEN_ID_1, expected_reward_rate);
}

#[test]
fn planned_distribution_state_update_should_not_happen_unless_triggered() {
    let mut world = setup_world_with_contract();

    send_set_distribution_plan_tx(&mut world, REWARD_TOKEN_ID_1, 0, 100, 100); // 1 tokens per round
    world.set_state_step(SetStateStep::new().block_round(100)); // advance to next round

    check_reward_rate(&mut world, &REWARD_TOKEN_ID_1, 0);
    check_last_distribution_round(&mut world, 0);
}

#[test]
fn existing_stakers_should_receive_rewards_with_no_state_update() {
    let mut world = setup_world_with_contract();

    send_set_distribution_plan_tx(&mut world, REWARD_TOKEN_ID_1, 0, 100, 100); // 1 tokens per round
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]); // triggers distribution

    check_reward_rate(&mut world, &REWARD_TOKEN_ID_1, 12345);
    check_last_distribution_round(&mut world, 999);
}
