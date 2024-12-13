use crate::errors::*;

pub const DEFAULT_PROPOSAL_DURATION_IN_SECONDS: u64 = 24 * 3600; // Allow proposals to be active for 1 day by default
pub const DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS: u64 = 3600; // Start proposal 1 hour after creation by default
pub const DIVISION_GUARD: u64 = 1000000000000000000; // 1e18

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// Representation of the lp_to_tro_ratio for a given lp token
/// (lp_token_id, tro_pool_supply, lp_pool_supply)
#[allow(type_alias_bounds)]
pub type LpToTroRatio<M: ManagedTypeApi> = MultiValue3<TokenIdentifier<M>, BigUint<M>, BigUint<M>>;

/// Representation of voting options
/// Invalid it not considered a valid vote thus is being completely
/// ignored from the voting validation logic
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub enum VoteDecision {
    Invalid = 0,
    Approve = 1,
    Abstain = 2,
    Reject = 3,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, TypeAbi)]
pub enum ProposalStatus {
    Invalid = 0,
    Pending = 1,
    Active = 2,
    Approved = 3,
    Rejected = 4,
    Failed = 5,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, TypeAbi)]
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
pub struct VoteContext<M: ManagedTypeApi> {
    pub decision: VoteDecision,
    pub voting_power: BigUint<M>,
    pub timestamp: u64,
    pub block: u64,
    pub epoch: u64,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, TypeAbi)]
pub struct ProposalVoteCount<M: ManagedTypeApi> {
    pub approve: BigUint<M>,
    pub abstain: BigUint<M>,
    pub reject: BigUint<M>,
    pub invalid: BigUint<M>,
}

#[multiversx_sc::module]
pub trait VotingModule: crate::storage::StorageModule + crate::events::EventsModule {
    #[only_owner]
    #[endpoint(createProposal)]
    #[allow_multiple_var_args]
    fn create_proposal(
        &self,
        title: ManagedBuffer,
        description: ManagedBuffer,
        min_voting_power_to_validate_vote: BigUint,
        start_time: OptionalValue<u64>,
        end_time: OptionalValue<u64>,
        lp_to_tro_ratios: MultiValueEncoded<LpToTroRatio<Self::Api>>,
    ) {
        let proposal_id = self.get_new_proposal_id();

        let start_time = start_time.into_option().unwrap_or(
            self.blockchain().get_block_timestamp() + DEFAULT_PROPOSAL_START_TIME_DELAY_IN_SECONDS,
        );
        let end_time = end_time
            .into_option()
            .unwrap_or(start_time + DEFAULT_PROPOSAL_DURATION_IN_SECONDS);

        self.require_time_range_is_valid(start_time, end_time);

        let proposal = Proposal {
            id: proposal_id,
            title: title.clone(),
            description: description.clone(),
            creator: self.blockchain().get_caller(),
            created_at: self.blockchain().get_block_timestamp(),
            start_time,
            end_time,
            min_voting_power_to_validate_vote: min_voting_power_to_validate_vote.clone(),
        };

        self.snapshot_lp_to_tro_ratio(proposal_id, lp_to_tro_ratios);

        self.proposals(proposal_id).set(proposal);

        self.emit_proposal_created_event(
            proposal_id,
            &title,
            &min_voting_power_to_validate_vote,
            start_time,
            end_time,
        );
    }

    #[endpoint(vote)]
    fn vote(&self, proposal_id: u64, decision: VoteDecision) {
        let caller = self.blockchain().get_caller();
        self.require_proposal_exists(proposal_id);
        self.require_proposal_active(proposal_id);
        self.require_user_has_not_voted(&caller, proposal_id);

        let voting_power = self.get_voting_power(&caller, proposal_id);

        require!(voting_power > 0, ERR_INSUFFICIENT_VOTING_POWER);

        self.proposal_votes(proposal_id, &decision)
            .update(|votes| *votes += &voting_power);

        let vote_context = self
            .get_proposal_vote_context(proposal_id, &caller)
            .into_option()
            .unwrap();
        self.emit_vote_event(&caller, proposal_id, vote_context);
    }

    fn get_proposal_vote_context(
        &self,
        proposal_id: u64,
        voter: &ManagedAddress,
    ) -> OptionalValue<VoteContext<Self::Api>> {
        if self.user_votes(voter, proposal_id).is_empty() {
            return OptionalValue::None;
        }

        OptionalValue::Some(self.user_votes(voter, proposal_id).get())
    }

    fn snapshot_lp_to_tro_ratio(
        &self,
        proposal_id: u64,
        lp_to_tro_ratios: MultiValueEncoded<LpToTroRatio<Self::Api>>,
    ) {
        for lp_to_tro_ratio in lp_to_tro_ratios {
            let (lp_token_id, tro_pool_supply, lp_pool_supply) = lp_to_tro_ratio.into_tuple();
            let ratio = tro_pool_supply * DIVISION_GUARD / lp_pool_supply;
            self.lp_to_tro_ratio(proposal_id, lp_token_id).set(ratio);
        }
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
            .proposal_votes(proposal.id, &VoteDecision::Approve)
            .get();
        let reject_votes = self
            .proposal_votes(proposal.id, &VoteDecision::Reject)
            .get();
        let abstain_votes = self
            .proposal_votes(proposal.id, &VoteDecision::Abstain)
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

    fn get_voting_power(&self, user: &ManagedAddress, proposal_id: u64) -> BigUint<Self::Api> {
        let tro_staked = self
            .users_stake(user, &self.tro_token_identifier().get())
            .get();
        let mut lp_voting_power = BigUint::zero();

        for lp_token in self.whitelisted_lp_token_identifiers().iter() {
            let staked_lp_balance = self.users_stake(user, &lp_token).get();
            let lp_to_tro_ratio = self.lp_to_tro_ratio(proposal_id, lp_token).get();
            let tro_equivalent = staked_lp_balance * lp_to_tro_ratio / DIVISION_GUARD;

            lp_voting_power += tro_equivalent;
        }

        lp_voting_power + tro_staked
    }

    fn require_time_range_is_valid(&self, start_time: u64, end_time: u64) {
        let current_timestamp = self.blockchain().get_block_timestamp();

        require!(start_time < end_time, ERR_INVALID_TIME_RANGE);
        require!(current_timestamp < start_time, ERR_INVALID_TIME_RANGE);
        require!(current_timestamp < end_time, ERR_INVALID_TIME_RANGE);
        require!(
            end_time - start_time >= DEFAULT_PROPOSAL_DURATION_IN_SECONDS,
            ERR_INVALID_TIME_RANGE
        );
        require!(
            end_time - start_time >= DEFAULT_PROPOSAL_DURATION_IN_SECONDS,
            ERR_INVALID_TIME_RANGE
        );
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

    fn require_no_proposal_ongoing(&self) {
        let last_proposal = self.last_proposal_id().get();
        if last_proposal == 0 {
            return;
        }

        let last_proposal_status = self.get_proposal_status(&self.proposals(last_proposal).get());

        require!(
            last_proposal_status != ProposalStatus::Active,
            ERR_PROPOSAL_ACTIVE
        );
    }

    fn require_user_has_not_voted(&self, user: &ManagedAddress, proposal_id: u64) {
        require!(
            self.user_votes(user, proposal_id).is_empty(),
            ERR_USER_ALREADY_VOTED
        );
    }

    fn get_proposal_vote_count(&self, proposal_id: u64) -> ProposalVoteCount<Self::Api> {
        ProposalVoteCount {
            approve: self
                .proposal_votes(proposal_id, &VoteDecision::Approve)
                .get(),
            abstain: self
                .proposal_votes(proposal_id, &VoteDecision::Abstain)
                .get(),
            reject: self
                .proposal_votes(proposal_id, &VoteDecision::Reject)
                .get(),
            invalid: self
                .proposal_votes(proposal_id, &VoteDecision::Invalid)
                .get(),
        }
    }

    // Counter for proposal ids
    #[view(getLastProposalId)]
    #[storage_mapper("last_proposal_id")]
    fn last_proposal_id(&self) -> SingleValueMapper<u64>;

    #[view(getProposal)]
    #[storage_mapper("proposals")]
    fn proposals(&self, proposal_id: u64) -> SingleValueMapper<Proposal<Self::Api>>;

    #[view(getProposalVotes)]
    #[storage_mapper("proposal_votes")]
    fn proposal_votes(
        &self,
        proposal_id: u64,
        decision: &VoteDecision,
    ) -> SingleValueMapper<BigUint>;

    #[view(getUserVote)]
    #[storage_mapper("user_votes")]
    fn user_votes(
        &self,
        user: &ManagedAddress,
        proposal_id: u64,
    ) -> SingleValueMapper<VoteContext<Self::Api>>;

    #[view(getLpToTroRatio)]
    #[storage_mapper("lp_to_tro_ratio")]
    fn lp_to_tro_ratio(
        &self,
        proposal_id: u64,
        lp_token: TokenIdentifier,
    ) -> SingleValueMapper<BigUint>;
}
