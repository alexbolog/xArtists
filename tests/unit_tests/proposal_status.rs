use crate::config::{
    LP_TOKEN_ID_1, LP_TOKEN_ID_2, LP_TOKEN_ID_3, OWNER_ADDRESS, TRO_TOKEN_ID, USER_ADDRESS,
};
use multiversx_sc::imports::MultiValue3;
use multiversx_sc::types::{
    BigUint, EsdtTokenPayment, ManagedBuffer, ManagedVec, MultiValueEncoded,
};
use tro_staking::admin::AdminModule;
use tro_staking::stake::StakeModule;
use tro_staking::voting::{Proposal, ProposalStatus, VotingModule};

use super::utils::init_staking_contract;

#[test]
fn test_time_based_proposal_status() {
    let contract = init_staking_contract();

    let proposal = Proposal {
        id: 1,
        title: ManagedBuffer::new_from_bytes(b"Test Proposal"),
        description: ManagedBuffer::new_from_bytes(b"This is a test proposal"),
        start_time: 1,
        end_time: 2,
        min_voting_power_to_validate_vote: BigUint::from(1000u64),
        creator: OWNER_ADDRESS.to_managed_address(),
        created_at: 1,
    };

    // Pending should be prior to the start time
    let status = contract.get_proposal_status(&proposal, 0);
    assert_eq!(status, ProposalStatus::Pending);

    // Active should be after the start time and before the end time
    let status = contract.get_proposal_status(&proposal, 1);
    assert_eq!(status, ProposalStatus::Active);

    // Failed should be after the end time
    let status = contract.get_proposal_status(&proposal, 3);
    assert_eq!(status, ProposalStatus::Failed);
}

#[test]
fn test_not_enough_votes_proposal_status() {
    let contract = init_staking_contract();

    let proposal = Proposal {
        id: 1,
        title: ManagedBuffer::new_from_bytes(b"Test Proposal"),
        description: ManagedBuffer::new_from_bytes(b"This is a test proposal"),
        start_time: 1,
        end_time: 2,
        min_voting_power_to_validate_vote: BigUint::from(1000u64),
        creator: OWNER_ADDRESS.to_managed_address(),
        created_at: 1,
    };

    contract.process_stake(
        &USER_ADDRESS.to_managed_address(),
        &ManagedVec::from_single_item(EsdtTokenPayment::new(
            TRO_TOKEN_ID.to_token_identifier(),
            0,
            BigUint::from(999u64), // stake just below the min voting power
        )),
    );
    contract.process_vote(
        &USER_ADDRESS.to_managed_address(),
        1,
        tro_staking::voting::VoteDecision::Approve,
    );

    let status = contract.get_proposal_status(&proposal, 1);
    assert_eq!(status, ProposalStatus::Active);

    let status = contract.get_proposal_status(&proposal, 3);
    assert_eq!(status, ProposalStatus::Failed);
}

#[test]
fn test_successful_proposal_status() {
    let contract = init_staking_contract();

    let proposal = Proposal {
        id: 1,
        title: ManagedBuffer::new_from_bytes(b"Test Proposal"),
        description: ManagedBuffer::new_from_bytes(b"This is a test proposal"),
        start_time: 1,
        end_time: 2,
        min_voting_power_to_validate_vote: BigUint::from(1000u64),
        creator: OWNER_ADDRESS.to_managed_address(),
        created_at: 1,
    };

    contract.process_stake(
        &USER_ADDRESS.to_managed_address(),
        &ManagedVec::from_single_item(EsdtTokenPayment::new(
            TRO_TOKEN_ID.to_token_identifier(),
            0,
            BigUint::from(1000u64), // stake enough to meet the min voting power
        )),
    );
    contract.process_vote(
        &USER_ADDRESS.to_managed_address(),
        1,
        tro_staking::voting::VoteDecision::Approve,
    );

    let status = contract.get_proposal_status(&proposal, 1);
    assert_eq!(status, ProposalStatus::Active);

    let status = contract.get_proposal_status(&proposal, 3);
    assert_eq!(status, ProposalStatus::Approved);
}

#[test]
fn test_rejected_proposal_status() {
    let contract = init_staking_contract();

    let proposal = Proposal {
        id: 1,
        title: ManagedBuffer::new_from_bytes(b"Test Proposal"),
        description: ManagedBuffer::new_from_bytes(b"This is a test proposal"),
        start_time: 1,
        end_time: 2,
        min_voting_power_to_validate_vote: BigUint::from(1000u64),
        creator: OWNER_ADDRESS.to_managed_address(),
        created_at: 1,
    };

    contract.process_stake(
        &USER_ADDRESS.to_managed_address(),
        &ManagedVec::from_single_item(EsdtTokenPayment::new(
            TRO_TOKEN_ID.to_token_identifier(),
            0,
            BigUint::from(1000u64), // stake enough to meet the min voting power
        )),
    );
    contract.process_vote(
        &USER_ADDRESS.to_managed_address(),
        1,
        tro_staking::voting::VoteDecision::Reject,
    );

    let status = contract.get_proposal_status(&proposal, 1);
    assert_eq!(status, ProposalStatus::Active);

    let status = contract.get_proposal_status(&proposal, 3);
    assert_eq!(status, ProposalStatus::Rejected);
}

#[test]
fn test_abstain_votes() {
    let contract = init_staking_contract();

    let proposal = Proposal {
        id: 1,
        title: ManagedBuffer::new_from_bytes(b"Test Proposal"),
        description: ManagedBuffer::new_from_bytes(b"This is a test proposal"),
        start_time: 1,
        end_time: 2,
        min_voting_power_to_validate_vote: BigUint::from(1000u64),
        creator: OWNER_ADDRESS.to_managed_address(),
        created_at: 1,
    };

    contract.process_stake(
        &USER_ADDRESS.to_managed_address(),
        &ManagedVec::from_single_item(EsdtTokenPayment::new(
            TRO_TOKEN_ID.to_token_identifier(),
            0,
            BigUint::from(1000u64), // stake enough to meet the min voting power
        )),
    );

    contract.process_vote(
        &USER_ADDRESS.to_managed_address(),
        1,
        tro_staking::voting::VoteDecision::Abstain,
    );

    let status = contract.get_proposal_status(&proposal, 1);
    assert_eq!(status, ProposalStatus::Active);

    let status = contract.get_proposal_status(&proposal, 3);
    assert_eq!(status, ProposalStatus::Rejected);

    contract.process_stake(
        &OWNER_ADDRESS.to_managed_address(),
        &ManagedVec::from_single_item(EsdtTokenPayment::new(
            TRO_TOKEN_ID.to_token_identifier(),
            0,
            BigUint::from(1000u64), // stake enough to meet the min voting power
        )),
    );

    contract.process_vote(
        &OWNER_ADDRESS.to_managed_address(),
        1,
        tro_staking::voting::VoteDecision::Approve,
    );

    let status = contract.get_proposal_status(&proposal, 3);
    assert_eq!(status, ProposalStatus::Approved);
}

#[test]
fn test_power_calculation() {
    let contract = init_staking_contract();

    let mut lp_token_identifiers = MultiValueEncoded::new();
    lp_token_identifiers.push(LP_TOKEN_ID_1.to_token_identifier());
    lp_token_identifiers.push(LP_TOKEN_ID_2.to_token_identifier());
    lp_token_identifiers.push(LP_TOKEN_ID_3.to_token_identifier());
    contract.add_whitelisted_lp_tokens(lp_token_identifiers);

    let proposal = Proposal {
        id: 1,
        title: ManagedBuffer::new_from_bytes(b"Test Proposal"),
        description: ManagedBuffer::new_from_bytes(b"This is a test proposal"),
        start_time: 1,
        end_time: 2,
        min_voting_power_to_validate_vote: BigUint::from(1000u64),
        creator: OWNER_ADDRESS.to_managed_address(),
        created_at: 1,
    };

    let mut lp_to_tro_ratio = MultiValueEncoded::new();
    // lp token 1 1:1
    lp_to_tro_ratio.push(MultiValue3((
        LP_TOKEN_ID_1.to_token_identifier(),
        BigUint::from(1000u64),
        BigUint::from(1000u64),
    )));
    // lp token 2 2:1
    lp_to_tro_ratio.push(MultiValue3((
        LP_TOKEN_ID_2.to_token_identifier(),
        BigUint::from(2000u64),
        BigUint::from(2000u64),
    )));
    // lp token 3 3:1
    lp_to_tro_ratio.push(MultiValue3((
        LP_TOKEN_ID_3.to_token_identifier(),
        BigUint::from(3000u64),
        BigUint::from(3000u64),
    )));

    contract.snapshot_lp_to_tro_ratio(1, lp_to_tro_ratio);

    contract.process_stake(
        &OWNER_ADDRESS.to_managed_address(),
        &ManagedVec::from_single_item(EsdtTokenPayment::new(
            TRO_TOKEN_ID.to_token_identifier(),
            0,
            BigUint::from(1000u64),
        )),
    );

    contract.process_stake(
        &USER_ADDRESS.to_managed_address(),
        &ManagedVec::from_single_item(EsdtTokenPayment::new(
            LP_TOKEN_ID_1.to_token_identifier(),
            0,
            BigUint::from(1000u64),
        )),
    );

    contract.process_stake(
        &USER_ADDRESS.to_managed_address(),
        &ManagedVec::from_single_item(EsdtTokenPayment::new(
            LP_TOKEN_ID_2.to_token_identifier(),
            0,
            BigUint::from(1000u64),
        )),
    );

    contract.process_stake(
        &USER_ADDRESS.to_managed_address(),
        &ManagedVec::from_single_item(EsdtTokenPayment::new(
            LP_TOKEN_ID_3.to_token_identifier(),
            0,
            BigUint::from(1000u64),
        )),
    );

    contract.process_vote(
        &OWNER_ADDRESS.to_managed_address(),
        1,
        tro_staking::voting::VoteDecision::Approve,
    );

    contract.process_vote(
        &USER_ADDRESS.to_managed_address(),
        1,
        tro_staking::voting::VoteDecision::Reject,
    );

    let status = contract.get_proposal_status(&proposal, 1);
    assert_eq!(status, ProposalStatus::Active);

    let status = contract.get_proposal_status(&proposal, 3);
    assert_eq!(status, ProposalStatus::Rejected);
}
