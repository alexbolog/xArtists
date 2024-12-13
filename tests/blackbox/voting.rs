use multiversx_sc_scenario::imports::*;
use tro_staking::{
    errors::*,
    proxy::VoteDecision,
    voting::{DEFAULT_PROPOSAL_DURATION_IN_SECONDS, DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS},
};

use crate::config::*;

use super::test_setup::setup_world_with_contract;

const PROPOSAL_TITLE: &[u8] = b"Test Proposal";
const PROPOSAL_DESCRIPTION: &[u8] = b"This is a test proposal";
const MIN_VOTING_POWER_TO_VALIDATE_VOTE: u64 = 1000;

#[test]
fn create_proposal_should_succeed() {
    let mut world = setup_world_with_contract();

    let title = ManagedBuffer::new_from_bytes(PROPOSAL_TITLE);
    let description = ManagedBuffer::new_from_bytes(PROPOSAL_DESCRIPTION);
    let min_voting_power_to_validate_vote = BigUint::from(MIN_VOTING_POWER_TO_VALIDATE_VOTE);
    let start_time = OptionalValue::<u64>::None;
    let end_time = OptionalValue::<u64>::None;

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .create_proposal(
            title,
            description,
            min_voting_power_to_validate_vote,
            start_time,
            end_time,
            MultiValueEncoded::new(),
        )
        .returns(ExpectStatus(0u64))
        .run();
}

#[test]
fn creating_multiple_simultaneous_proposals_should_succeed() {
    let mut world = setup_world_with_contract();

    let title = ManagedBuffer::new_from_bytes(PROPOSAL_TITLE);
    let description = ManagedBuffer::new_from_bytes(PROPOSAL_DESCRIPTION);
    let min_voting_power_to_validate_vote = BigUint::from(MIN_VOTING_POWER_TO_VALIDATE_VOTE);
    let start_time = OptionalValue::<u64>::None;
    let end_time = OptionalValue::<u64>::None;

    for _i in 0..10 {
        world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(tro_staking::proxy::TroStakingProxy)
            .create_proposal(
                title.clone(),
                description.clone(),
                min_voting_power_to_validate_vote.clone(),
                start_time.clone(),
                end_time.clone(),
                MultiValueEncoded::new(),
            )
            .returns(ExpectStatus(0u64))
            .run();
    }
}

#[test]
fn create_proposal_with_invalid_time_range_should_fail() {
    let mut world = setup_world_with_contract();

    let title = ManagedBuffer::new_from_bytes(PROPOSAL_TITLE);
    let description = ManagedBuffer::new_from_bytes(PROPOSAL_DESCRIPTION);
    let min_voting_power_to_validate_vote = BigUint::from(MIN_VOTING_POWER_TO_VALIDATE_VOTE);

    let block_timestamp = 100;

    world.set_state_step(SetStateStep::new().block_timestamp(block_timestamp));

    let test_cases = [
        (
            OptionalValue::Some(block_timestamp - 1),
            OptionalValue::Some(block_timestamp + DEFAULT_PROPOSAL_DURATION_IN_SECONDS),
        ),
        (
            OptionalValue::Some(block_timestamp),
            OptionalValue::Some(block_timestamp),
        ),
        (
            OptionalValue::Some(block_timestamp),
            OptionalValue::Some(block_timestamp + DEFAULT_PROPOSAL_DURATION_IN_SECONDS - 1),
        ),
        (
            OptionalValue::Some(block_timestamp - 1),
            OptionalValue::None,
        ),
        (OptionalValue::None, OptionalValue::Some(block_timestamp)),
        (
            OptionalValue::None,
            OptionalValue::Some(block_timestamp - 1),
        ),
    ];

    for (start_time, end_time) in test_cases {
        world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(tro_staking::proxy::TroStakingProxy)
            .create_proposal(
                title.clone(),
                description.clone(),
                min_voting_power_to_validate_vote.clone(),
                start_time.clone(),
                end_time.clone(),
                MultiValueEncoded::new(),
            )
            .returns(ExpectMessage(ERR_INVALID_TIME_RANGE))
            .run();
    }
}

#[test]
fn voting_on_proposal_should_succeed() {
    let mut world = setup_world_with_contract();

    create_proposal(&mut world);
    add_tro_stake(&mut world);

    world.set_state_step(
        SetStateStep::new().block_timestamp(DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS + 1),
    );

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .vote(1u64, VoteDecision::Approve)
        .returns(ExpectStatus(0u64))
        .run();
}

#[test]
fn voting_on_multiple_active_proposals_should_succeed() {
    let mut world = setup_world_with_contract();

    create_proposal(&mut world);
    create_proposal(&mut world);
    create_proposal(&mut world);
    add_tro_stake(&mut world);

    world.set_state_step(
        SetStateStep::new().block_timestamp(DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS + 1),
    );

    for proposal_id in 1..=3u64 {
        world
            .tx()
            .from(USER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(tro_staking::proxy::TroStakingProxy)
            .vote(proposal_id, VoteDecision::Approve)
            .returns(ExpectStatus(0u64))
            .run();
    }
}

#[test]
fn voting_on_not_existing_proposal_should_fail() {
    let mut world = setup_world_with_contract();

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .vote(1u64, VoteDecision::Approve)
        .returns(ExpectMessage(ERR_PROPOSAL_NOT_FOUND))
        .run();
}

#[test]
fn voting_on_pending_proposal_should_fail() {
    let mut world = setup_world_with_contract();

    create_proposal(&mut world);
    add_tro_stake(&mut world);

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .vote(1u64, VoteDecision::Approve)
        .returns(ExpectMessage(ERR_PROPOSAL_NOT_ACTIVE))
        .run();
}

#[test]
fn voting_on_ended_proposal_should_fail() {
    let mut world = setup_world_with_contract();

    create_proposal(&mut world);
    add_tro_stake(&mut world);

    world.set_state_step(SetStateStep::new().block_timestamp(
        DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS + DEFAULT_PROPOSAL_DURATION_IN_SECONDS + 1,
    ));

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .vote(1u64, VoteDecision::Approve)
        .returns(ExpectMessage(ERR_PROPOSAL_NOT_ACTIVE))
        .run();
}

#[test]
fn voting_on_proposal_with_insufficient_voting_power_should_fail() {
    let mut world = setup_world_with_contract();

    create_proposal(&mut world);

    world.set_state_step(
        SetStateStep::new().block_timestamp(DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS + 1),
    );

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .vote(1u64, VoteDecision::Approve)
        .returns(ExpectMessage(ERR_INSUFFICIENT_VOTING_POWER))
        .run();
}

fn create_proposal(world: &mut ScenarioWorld) {
    let title = ManagedBuffer::new_from_bytes(PROPOSAL_TITLE);
    let description = ManagedBuffer::new_from_bytes(PROPOSAL_DESCRIPTION);
    let min_voting_power_to_validate_vote = BigUint::from(MIN_VOTING_POWER_TO_VALIDATE_VOTE);
    let start_time = OptionalValue::<u64>::None;
    let end_time = OptionalValue::<u64>::None;

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .create_proposal(
            title,
            description,
            min_voting_power_to_validate_vote,
            start_time,
            end_time,
            MultiValueEncoded::new(),
        )
        .returns(ExpectStatus(0u64))
        .run();
}

fn add_tro_stake(world: &mut ScenarioWorld) {
    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .stake()
        .payment(EsdtTokenPayment::new(
            TRO_TOKEN_ID.to_token_identifier(),
            0,
            BigUint::from(1000u64),
        ))
        .returns(ExpectStatus(0u64))
        .run();
}
