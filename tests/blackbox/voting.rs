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
    add_stake(&mut world, TRO_TOKEN_ID, 1000);

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
    add_stake(&mut world, TRO_TOKEN_ID, 1000);

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
    add_stake(&mut world, TRO_TOKEN_ID, 1000);

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
    add_stake(&mut world, TRO_TOKEN_ID, 1000);

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

#[test]
fn voting_power_should_be_calculated_correctly() {
    let mut world = setup_world_with_contract();

    add_stake(&mut world, TRO_TOKEN_ID, 1000);
    add_stake(&mut world, LP_TOKEN_ID_1, 1000);
    add_stake(&mut world, LP_TOKEN_ID_2, 1000);
    add_stake(&mut world, LP_TOKEN_ID_3, 1000);

    let expected_tro_voting_power = BigUint::from(1000u64 * (1 + 1 + 2 + 3));

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
        .returns(ExpectStatus(0u64))
        .run();

    world
        .query()
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .proposal_votes(1u64, VoteDecision::Approve)
        .returns(ExpectValue(expected_tro_voting_power.clone()))
        .run();

    world
        .query()
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .get_voting_power_view(USER_ADDRESS.to_managed_address(), OptionalValue::Some(1u64))
        .returns(ExpectValue(expected_tro_voting_power))
        .run();
}

fn create_proposal(world: &mut ScenarioWorld) {
    let title = ManagedBuffer::new_from_bytes(PROPOSAL_TITLE);
    let description = ManagedBuffer::new_from_bytes(PROPOSAL_DESCRIPTION);
    let min_voting_power_to_validate_vote = BigUint::from(MIN_VOTING_POWER_TO_VALIDATE_VOTE);
    let start_time = DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS;
    let end_time =
        DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS + DEFAULT_PROPOSAL_DURATION_IN_SECONDS;

    let mut lp_to_tro_balances = MultiValueEncoded::new();
    lp_to_tro_balances.push(MultiValue3((
        LP_TOKEN_ID_1.to_token_identifier(), // 1:1 tro
        BigUint::from(1000u64),
        BigUint::from(1000u64),
    )));
    lp_to_tro_balances.push(MultiValue3((
        LP_TOKEN_ID_2.to_token_identifier(),
        BigUint::from(2000u64), // 2:1 tro
        BigUint::from(1000u64),
    )));
    lp_to_tro_balances.push(MultiValue3((
        LP_TOKEN_ID_3.to_token_identifier(),
        BigUint::from(3000u64), // 3:1 tro
        BigUint::from(1000u64),
    )));

    world.set_state_step(SetStateStep::new().block_timestamp(1));

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .create_proposal(
            title,
            description,
            min_voting_power_to_validate_vote,
            OptionalValue::Some(start_time),
            OptionalValue::Some(end_time),
            lp_to_tro_balances,
        )
        .returns(ExpectStatus(0u64))
        .run();
}

fn add_stake(world: &mut ScenarioWorld, token_id: TestTokenIdentifier, amount: u64) {
    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .stake()
        .payment(EsdtTokenPayment::new(
            token_id.to_token_identifier(),
            0,
            BigUint::from(amount),
        ))
        .returns(ExpectStatus(0u64))
        .run();
}
