use crate::errors::{
    ERR_INSUFFICIENT_VOTING_POWER, ERR_PROPOSAL_NOT_ACTIVE, ERR_PROPOSAL_NOT_FOUND,
};

pub const DEFAULT_PROPOSAL_DURATION_IN_SECONDS: u64 = 24 * 3600; // Allow proposals to be active for 1 day by default
pub const DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS: u64 = 3600; // Start proposal 1 hour after creation by default

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub enum VoteDecision {
    Invalid = 1,
    Approve = 2,
    Abstain = 3,
    Reject = 4,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, TypeAbi)]
pub enum ProposalStatus {
    Pending = 1,
    Active = 2,
    Approved = 3,
    Rejected = 4,
    Failed = 5,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct Proposal<M: ManagedTypeApi> {
    pub id: u64,
    pub title: ManagedBuffer<M>,
    pub description: ManagedBuffer<M>,
    pub creator: ManagedAddress<M>,
    pub created_at: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub min_voting_power_to_validate_vote: BigUint<M>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct ProposalViewStatus<M: ManagedTypeApi> {
    pub proposal: Proposal<M>,
    pub status: ProposalStatus,
    pub votes: BigUint<M>,
}

#[multiversx_sc::module]
pub trait VotingModule {
    #[only_owner]
    #[endpoint(createProposal)]
    fn create_proposal(
        &self,
        title: ManagedBuffer,
        description: ManagedBuffer,
        min_voting_power_to_validate_vote: BigUint,
        start_time: OptionalValue<u64>,
        end_time: OptionalValue<u64>,
    ) {
        let proposal_id = self.get_new_proposal_id();

        let start_time = start_time.into_option().unwrap_or(
            self.blockchain().get_block_timestamp() + DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS,
        );
        let end_time = end_time
            .into_option()
            .unwrap_or(start_time + DEFAULT_PROPOSAL_DURATION_IN_SECONDS);

        let proposal = Proposal {
            id: proposal_id,
            title,
            description,
            creator: self.blockchain().get_caller(),
            created_at: self.blockchain().get_block_timestamp(),
            start_time,
            end_time,
            min_voting_power_to_validate_vote,
        };

        self.proposals(proposal_id).set(proposal);
    }

    #[endpoint(vote)]
    fn vote(&self, proposal_id: u64, decision: VoteDecision) {
        let caller = self.blockchain().get_caller();
        self.require_proposal_exists(proposal_id);
        self.require_proposal_active(proposal_id);

        let voting_power = self.get_voting_power(&caller);

        require!(voting_power > 0, ERR_INSUFFICIENT_VOTING_POWER);

        self.proposal_votes(proposal_id, decision)
            .update(|votes| *votes += &voting_power);
    }

    // TODO: add unit tests for this
    fn get_proposal_status(&self, proposal: &Proposal<Self::Api>) -> ProposalStatus {
        let block_timestamp = self.blockchain().get_block_timestamp();
        if block_timestamp < proposal.start_time {
            ProposalStatus::Pending
        } else if block_timestamp > proposal.end_time {
            self.get_proposal_vote_result(proposal)
        } else {
            ProposalStatus::Active
        }
    }

    fn get_proposal_vote_result(&self, proposal: &Proposal<Self::Api>) -> ProposalStatus {
        let approve_votes = self
            .proposal_votes(proposal.id, VoteDecision::Approve)
            .get();
        let reject_votes = self.proposal_votes(proposal.id, VoteDecision::Reject).get();
        let abstain_votes = self
            .proposal_votes(proposal.id, VoteDecision::Abstain)
            .get();

        let total_votes = &approve_votes + &reject_votes + &abstain_votes;

        if total_votes > proposal.min_voting_power_to_validate_vote {
            if approve_votes > reject_votes {
                ProposalStatus::Approved
            } else {
                ProposalStatus::Rejected
            }
        } else {
            ProposalStatus::Failed
        }
    }

    fn get_new_proposal_id(&self) -> u64 {
        let new_proposal_id = self.last_proposal_id().get() + 1;
        self.last_proposal_id().set(new_proposal_id);

        new_proposal_id
    }

    fn require_proposal_exists(&self, proposal_id: u64) {
        require!(
            !self.proposals(proposal_id).is_empty(),
            ERR_PROPOSAL_NOT_FOUND
        );
    }

    fn require_proposal_active(&self, proposal_id: u64) {
        require!(
            self.get_proposal_status(&self.proposals(proposal_id).get()) == ProposalStatus::Active,
            ERR_PROPOSAL_NOT_ACTIVE
        );
    }

    fn get_voting_power(&self, user: &ManagedAddress) -> BigUint<Self::Api> {
        todo!()
    }

    // Counter for proposal ids
    #[view(getLastProposalId)]
    #[storage_mapper("last_proposal_id")]
    fn last_proposal_id(&self) -> SingleValueMapper<u64>;

    #[view(getProposals)]
    #[storage_mapper("proposals")]
    fn proposals(&self, proposal_id: u64) -> SingleValueMapper<Proposal<Self::Api>>;

    #[view(getProposalVotes)]
    #[storage_mapper("proposal_votes")]
    fn proposal_votes(
        &self,
        proposal_id: u64,
        decision: VoteDecision,
    ) -> SingleValueMapper<BigUint>;
}
