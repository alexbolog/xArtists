use multiversx_sc::types::TestTokenIdentifier;
use multiversx_sc_scenario::imports::*;
use multiversx_sc_scenario::{ExpectValue, ScenarioWorld};

use crate::config::{OWNER_ADDRESS, SC_ADDRESS};

////////////////////////////////////////////////////////////
/// Query Helpers
////////////////////////////////////////////////////////////

pub fn check_staked_amount(
    world: &mut ScenarioWorld,
    user: &TestAddress,
    token_id: &TestTokenIdentifier,
    nonce: u64,
    expected_amount: u64,
) {
    world
        .query()
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .get_stake_quantity(user.to_address(), token_id.to_token_identifier(), nonce)
        .returns(ExpectValue(expected_amount))
        .run();
}

pub fn check_user_staking_score(
    world: &mut ScenarioWorld,
    user: &TestAddress,
    expected_score: u64,
) {
    world
        .query()
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .get_user_staking_score(user.to_address())
        .returns(ExpectValue(expected_score))
        .run();
}

pub fn check_aggregated_staking_score(world: &mut ScenarioWorld, expected_score: u64) {
    world
        .query()
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .get_aggregated_staking_score()
        .returns(ExpectValue(expected_score))
        .run();
}

pub fn check_reward_rate(
    world: &mut ScenarioWorld,
    token_id: &TestTokenIdentifier,
    expected_rate: RustBigUint,
) {
    world
        .query()
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .get_reward_rate(token_id.to_token_identifier())
        .returns(ExpectValue(expected_rate))
        .run();
}

pub fn check_distribution_amount_per_round(
    world: &mut ScenarioWorld,
    start_round: u64,
    end_round: u64,
    total_distribution_amount: RustBigUint,
    expected_amount_per_round: RustBigUint,
) {
    world
        .query()
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .get_amount_per_round(start_round, end_round, total_distribution_amount)
        .returns(ExpectValue(expected_amount_per_round))
        .run();
}

pub fn check_last_distribution_round(world: &mut ScenarioWorld, expected_round: u64) {
    world
        .query()
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .get_last_distribution_round()
        .returns(ExpectValue(expected_round))
        .run();
}

pub fn check_if_token_is_reward_token(world: &mut ScenarioWorld, token_id: &TestTokenIdentifier) {
    world
        .query()
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .is_reward_token(token_id.to_token_identifier())
        .returns(ExpectValue(true))
        .run();
}

pub fn check_pending_reward(
    world: &mut ScenarioWorld,
    user: &TestAddress,
    token_id: &TestTokenIdentifier,
    expected_reward: RustBigUint,
) {
    world
        .query()
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .get_pending_token_reward(user.to_address(), token_id.to_token_identifier())
        .returns(ExpectValue(expected_reward))
        .run();
}

////////////////////////////////////////////////////////////
/// Transaction Helpers
////////////////////////////////////////////////////////////

/// Owner transactions
pub fn send_enable_staking_tx(world: &mut ScenarioWorld) {
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .enable_staking()
        .returns(ExpectStatus(0u64))
        .run();
}

pub fn send_disable_staking_tx(world: &mut ScenarioWorld) {
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .disable_staking()
        .returns(ExpectStatus(0u64))
        .run();
}

pub fn send_allow_collection_tx(world: &mut ScenarioWorld, token_id: &TestTokenIdentifier) {
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .allow_collections(MultiValueManagedVec::from_single_item(
            token_id.to_token_identifier(),
        ))
        .returns(ExpectStatus(0u64))
        .run();
}

pub fn send_disallow_collection_tx(world: &mut ScenarioWorld, token_id: &TestTokenIdentifier) {
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .disallow_collections(MultiValueManagedVec::from_single_item(
            token_id.to_token_identifier(),
        ))
        .returns(ExpectStatus(0u64))
        .run();
}

/// User transactions
pub fn send_stake_tx(
    world: &mut ScenarioWorld,
    user: &TestAddress,
    payments: &[&(TestTokenIdentifier, u64, u64)],
) {
    let mut payments_arg = MultiEsdtPayment::new();

    for (token_id, nonce, amount) in payments.iter() {
        payments_arg.push(EsdtTokenPayment::new(
            token_id.to_token_identifier(),
            *nonce,
            managed_biguint!(*amount),
        ));
    }

    world
        .tx()
        .from(user.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .stake()
        .multi_esdt(payments_arg)
        .returns(ExpectStatus(0u64))
        .run();
}

pub fn send_unstake_tx(
    world: &mut ScenarioWorld,
    user: &TestAddress,
    assets: &[&(TestTokenIdentifier, u64, u64)],
) {
    let mut assets_arg = MultiValueManagedVec::new();

    for (token_id, nonce, amount) in assets.iter() {
        assets_arg.push(EsdtTokenPayment::new(
            token_id.to_token_identifier(),
            *nonce,
            managed_biguint!(*amount),
        ));
    }

    world
        .tx()
        .from(user.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .unstake(assets_arg)
        .returns(ExpectStatus(0u64))
        .run();
}

pub fn send_set_collection_score_tx(
    world: &mut ScenarioWorld,
    token_id: &TestTokenIdentifier,
    score: u64,
) {
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .set_collection_score(token_id.to_token_identifier(), score)
        .returns(ExpectStatus(0u64))
        .run();
}

pub fn send_set_collection_nonce_score_tx(
    world: &mut ScenarioWorld,
    token_id: &TestTokenIdentifier,
    nonce: u64,
    score: u64,
) {
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .set_collection_nonce_score(token_id.to_token_identifier(), nonce, score)
        .returns(ExpectStatus(0u64))
        .run();
}

pub fn send_distribute_rewards_tx(
    world: &mut ScenarioWorld,
    token_id: TestTokenIdentifier,
    amount: u64,
) {
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .distribute_rewards()
        .with_esdt_transfer(EsdtTokenPayment::new(
            token_id.to_token_identifier(),
            0u64,
            managed_biguint!(amount),
        ))
        .returns(ExpectStatus(0u64))
        .run();
}

pub fn send_set_distribution_plan_tx(
    world: &mut ScenarioWorld,
    token_id: TestTokenIdentifier,
    start_round: u64,
    end_round: u64,
    total_amount: u64,
) {
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .create_distribution_plan(start_round, end_round)
        .with_esdt_transfer(EsdtTokenPayment::new(
            token_id.to_token_identifier(),
            0u64,
            managed_biguint!(total_amount),
        ))
        .run();
}

pub fn send_claim_rewards_tx(world: &mut ScenarioWorld, user: &TestAddress) {
    world
        .tx()
        .from(user.to_address())
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .claim_rewards()
        .returns(ExpectStatus(0u64))
        .run();
}
