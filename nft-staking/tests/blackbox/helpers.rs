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
    let _ = world
        .query()
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .get_stake_quantity(user.to_address(), token_id.to_token_identifier(), nonce)
        .returns(ExpectValue(expected_amount));
}

pub fn check_user_staking_score(
    world: &mut ScenarioWorld,
    user: &TestAddress,
    expected_score: u64,
) {
    let _ = world
        .query()
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .get_user_staking_score(user.to_address())
        .returns(ExpectValue(expected_score));
}

pub fn check_aggregated_staking_score(world: &mut ScenarioWorld, expected_score: u64) {
    let _ = world
        .query()
        .to(SC_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .get_aggregated_staking_score()
        .returns(ExpectValue(expected_score));
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
            BigUint::from(*amount),
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
            BigUint::from(*amount),
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
