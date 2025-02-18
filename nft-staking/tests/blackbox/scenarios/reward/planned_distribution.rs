use multiversx_sc_scenario::{imports::SetStateStep, rust_biguint};
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
    config::{NFT_TOKEN_ID, OWNER_ADDRESS, REWARD_TOKEN_ID_1, SFT_TOKEN_ID, USER_ADDRESS},
};

#[test]
fn planned_reward_distribution_round_distribution_amount_calculation_should_work() {
    let mut world = setup_world_with_contract();

    check_distribution_amount_per_round(
        &mut world,
        0,
        10,
        rust_biguint!(REWARD_RATE_DENOMINATION),
        rust_biguint!(REWARD_RATE_DENOMINATION / 10) * rust_biguint!(REWARD_RATE_DENOMINATION),
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
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]); // single staker, takes entire reward pool, triggers distribution but gets nothing as it's the first round

    world.set_state_step(SetStateStep::new().block_round(1)); // advance to next round

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 2, 1)]); // trigger distribution
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 3, 1)]); // trigger another distribution
    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 4, 1)]); // trigger one more distribution

    // at this point, the state is:
    // - 2 tokens distributed across 2 rounds (round 0 and round 1)
    // - total staked score is 2, and owned by the same single staker
    // - reward rate is 1 token per round

    let expected_reward_rate = REWARD_RATE_DENOMINATION / DEFAULT_NFT_SCORE; // 1 token per round
    check_reward_rate(
        &mut world,
        &REWARD_TOKEN_ID_1,
        rust_biguint!(expected_reward_rate),
    );
}

#[test]
fn planned_distribution_state_update_should_not_happen_unless_triggered() {
    let mut world = setup_world_with_contract();

    send_set_distribution_plan_tx(&mut world, REWARD_TOKEN_ID_1, 0, 100, 100); // 1 tokens per round
    world.set_state_step(SetStateStep::new().block_round(100)); // advance to next round

    // The next check incorrectly fails with 0x00 != 0x0, waiting for a fix in a future release
    // check_reward_rate(&mut world, &REWARD_TOKEN_ID_1, rust_biguint!(0));
    check_last_distribution_round(&mut world, 0);
}

#[test]
fn existing_stakers_should_receive_rewards_with_no_state_update() {
    let mut world = setup_world_with_contract();

    send_stake_tx(&mut world, &USER_ADDRESS, &[&(NFT_TOKEN_ID, 1, 1)]);
    send_set_distribution_plan_tx(&mut world, REWARD_TOKEN_ID_1, 0, 100, 100); // 1 tokens per round

    world.set_state_step(SetStateStep::new().block_round(1)); // advance to next round

    send_stake_tx(&mut world, &OWNER_ADDRESS, &[&(SFT_TOKEN_ID, 1, 1)]); // trigger distribution from another account
    check_reward_rate(
        &mut world,
        &REWARD_TOKEN_ID_1,
        rust_biguint!(REWARD_RATE_DENOMINATION / DEFAULT_NFT_SCORE),
    );
    check_last_distribution_round(&mut world, 1);
}
