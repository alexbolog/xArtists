use multiversx_sc::derive_imports::*;
#[allow(unused_imports)]
use multiversx_sc::imports::*;

use crate::voting::{
    FullProposalContext, Proposal, ProposalStatus, ProposalVoteCount, VoteContext,
};

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct StakingContext<M: ManagedTypeApi> {
    pub users_stake: ManagedVec<M, EsdtTokenPayment<M>>,
    pub last_proposals_context: ProposalContext<M>,
    pub active_proposal_ids: ManagedVec<M, u64>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct ProposalContext<M: ManagedTypeApi> {
    pub proposal: Proposal<M>,
    pub users_voting_power: BigUint<M>,
    pub users_vote: Option<VoteContext<M>>,
    pub proposal_status: ProposalStatus,
    pub proposal_vote_count: ProposalVoteCount<M>,
}

#[multiversx_sc::module]
pub trait ViewsModule:
    crate::storage::StorageModule + crate::voting::VotingModule + crate::events::EventsModule
{
    #[view(getVotingPower)]
    fn get_voting_power_view(
        &self,
        user: &ManagedAddress,
        proposal_id: OptionalValue<u64>,
    ) -> BigUint {
        let proposal_id = proposal_id
            .into_option()
            .unwrap_or(self.last_proposal_id().get());
        self.get_voting_power(user, proposal_id)
    }

    #[view(getStakingContext)]
    fn get_staking_context(
        &self,
        user: OptionalValue<ManagedAddress>,
    ) -> OptionalValue<StakingContext<Self::Api>> {
        let last_proposals_context =
            self.get_proposal_context(self.last_proposal_id().get(), user.clone());
        let user = user.into_option().unwrap_or(ManagedAddress::zero());
        let users_stake = self.get_user_complete_stake(user);
        let active_proposal_ids = self.get_active_proposal_ids();

        OptionalValue::Some(StakingContext {
            users_stake,
            last_proposals_context,
            active_proposal_ids,
        })
    }

    #[view(getUserCompleteStake)]
    fn get_user_complete_stake(
        &self,
        user: ManagedAddress,
    ) -> ManagedVec<Self::Api, EsdtTokenPayment<Self::Api>> {
        if user == ManagedAddress::zero() {
            return ManagedVec::new();
        }

        let tro_stake_info = self.get_user_stake(&user, &self.tro_token_identifier().get());
        let mut users_stake = ManagedVec::new();

        if tro_stake_info.amount > 0 {
            users_stake.push(tro_stake_info);
        }

        for token_id in self.whitelisted_lp_token_identifiers().iter() {
            let lp_stake_info = self.get_user_stake(&user, &token_id);
            users_stake.push(lp_stake_info);
        }

        users_stake
    }

    #[view(getUserStake)]
    fn get_user_stake(
        &self,
        user: &ManagedAddress,
        token_identifier: &TokenIdentifier,
    ) -> EsdtTokenPayment<Self::Api> {
        EsdtTokenPayment::new(
            token_identifier.clone(),
            0u64,
            self.users_stake(user, token_identifier).get(),
        )
    }

    #[view(getProposalContext)]
    fn get_proposal_context(
        &self,
        proposal_id: u64,
        user: OptionalValue<ManagedAddress>,
    ) -> ProposalContext<Self::Api> {
        let (users_voting_power, users_vote) = if user.is_some() {
            let user = user.into_option().unwrap();
            let voting_power = self.get_voting_power(&user, proposal_id);
            let voting_context = self
                .get_proposal_vote_context(proposal_id, &user)
                .into_option();
            (voting_power, voting_context)
        } else {
            (BigUint::zero(), None)
        };

        ProposalContext {
            proposal: self.proposals(proposal_id).get(),
            users_voting_power,
            users_vote,
            proposal_status: ProposalStatus::Active,
            proposal_vote_count: self.get_proposal_vote_count(proposal_id),
        }
    }

    #[view(getActiveProposalIds)]
    fn get_active_proposal_ids(&self) -> ManagedVec<Self::Api, u64> {
        let mut active_proposal_ids = ManagedVec::new();

        let mut last_proposal_id = self.last_proposal_id().get();
        while last_proposal_id > 0 {
            let proposal_status = self.get_proposal_status(
                &self.proposals(last_proposal_id).get(),
                self.blockchain().get_block_timestamp(),
            );
            if proposal_status == ProposalStatus::Active {
                active_proposal_ids.push(last_proposal_id);
            }
            last_proposal_id -= 1;
        }

        active_proposal_ids
    }

    #[view(getProposalStatus)]
    fn get_proposal_status_view(&self, proposal_id: u64) -> ProposalStatus {
        self.get_proposal_status(
            &self.proposals(proposal_id).get(),
            self.blockchain().get_block_timestamp(),
        )
    }

    #[view(getProposalVoteContext)]
    fn get_proposal_vote_context_view(
        &self,
        proposal_id: u64,
        voter: &ManagedAddress,
    ) -> OptionalValue<VoteContext<Self::Api>> {
        self.get_proposal_vote_context(proposal_id, voter)
    }

    #[view(getProposalVoteCount)]
    fn get_proposal_vote_count_view(&self, proposal_id: u64) -> ProposalVoteCount<Self::Api> {
        self.get_proposal_vote_count(proposal_id)
    }

    #[view(getAllProposals)]
    fn get_all_proposals(
        &self,
        user: OptionalValue<ManagedAddress>,
    ) -> ManagedVec<Self::Api, FullProposalContext<Self::Api>> {
        let mut last_proposal_id = self.last_proposal_id().get();
        let mut proposals = ManagedVec::new();
        let user = user.into_option().unwrap_or(ManagedAddress::zero());

        while last_proposal_id > 0 {
            proposals.push(FullProposalContext {
                proposal: self.proposals(last_proposal_id).get(),
                users_voting_power: self.get_voting_power(&user, last_proposal_id),
                users_vote: self
                    .get_proposal_vote_context(last_proposal_id, &user)
                    .into_option(),
                proposal_status: self.get_proposal_status(
                    &self.proposals(last_proposal_id).get(),
                    self.blockchain().get_block_timestamp(),
                ) as u8,
                proposal_vote_count: self.get_proposal_vote_count(last_proposal_id),
            });
            last_proposal_id -= 1;
        }

        proposals
    }
}
