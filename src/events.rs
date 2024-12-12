use crate::voting::VoteDecision;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode)]
pub struct StakeEvent<M: ManagedTypeApi> {
    caller: ManagedAddress<M>,
    payments: MultiEsdtPayment<M>,
}

#[allow(type_alias_bounds)]
pub type UnstakeEvent<M: ManagedTypeApi> = StakeEvent<M>;

#[derive(TypeAbi, TopEncode)]
pub struct ProposalCreatedEvent<M: ManagedTypeApi> {
    creator: ManagedAddress<M>,
    proposal_id: u64,
    title: ManagedBuffer<M>,
    min_voting_power: BigUint<M>,
    start_time: u64,
    end_time: u64,
}

#[derive(TypeAbi, TopEncode)]
pub struct VoteEvent<M: ManagedTypeApi> {
    voter: ManagedAddress<M>,
    proposal_id: u64,
    decision: VoteDecision,
    voting_power: BigUint<M>,
}

#[multiversx_sc::module]
pub trait EventsModule: crate::storage::StorageModule {
    fn emit_stake_event(&self, caller: &ManagedAddress, payments: &MultiEsdtPayment<Self::Api>) {
        let epoch = self.blockchain().get_block_epoch();
        let stake_event = StakeEvent {
            caller: caller.clone(),
            payments: payments.clone(),
        };

        self.stake_event(
            caller,
            epoch,
            self.blockchain().get_block_nonce(),
            self.blockchain().get_block_timestamp(),
            &stake_event,
        );
    }

    fn emit_unstake_event(&self, caller: &ManagedAddress, payments: &MultiEsdtPayment<Self::Api>) {
        let epoch = self.blockchain().get_block_epoch();
        let unstake_event = UnstakeEvent {
            caller: caller.clone(),
            payments: payments.clone(),
        };

        self.unstake_event(
            caller,
            epoch,
            self.blockchain().get_block_nonce(),
            self.blockchain().get_block_timestamp(),
            &unstake_event,
        );
    }

    fn emit_proposal_created_event(
        &self,
        proposal_id: u64,
        title: &ManagedBuffer<Self::Api>,
        min_voting_power: &BigUint<Self::Api>,
        start_time: u64,
        end_time: u64,
    ) {
        let event = ProposalCreatedEvent {
            creator: self.blockchain().get_caller(),
            proposal_id,
            title: title.clone(),
            min_voting_power: min_voting_power.clone(),
            start_time,
            end_time,
        };

        self.proposal_created_event(
            &self.blockchain().get_caller(),
            self.blockchain().get_block_epoch(),
            self.blockchain().get_block_nonce(),
            self.blockchain().get_block_timestamp(),
            &event,
        );
    }

    fn emit_vote_event(
        &self,
        proposal_id: u64,
        decision: VoteDecision,
        voting_power: &BigUint<Self::Api>,
    ) {
        let event = VoteEvent {
            voter: self.blockchain().get_caller(),
            proposal_id,
            decision,
            voting_power: voting_power.clone(),
        };

        self.vote_event(
            &self.blockchain().get_caller(),
            self.blockchain().get_block_epoch(),
            self.blockchain().get_block_nonce(),
            self.blockchain().get_block_timestamp(),
            &event,
        );
    }

    #[event("stake")]
    fn stake_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] block: u64,
        #[indexed] timestamp: u64,
        stake_event: &StakeEvent<Self::Api>,
    );

    #[event("unstake")]
    fn unstake_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] block: u64,
        #[indexed] timestamp: u64,
        unstake_event: &UnstakeEvent<Self::Api>,
    );

    #[event("proposalCreated")]
    fn proposal_created_event(
        &self,
        #[indexed] creator: &ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] block: u64,
        #[indexed] timestamp: u64,
        event: &ProposalCreatedEvent<Self::Api>,
    );

    #[event("vote")]
    fn vote_event(
        &self,
        #[indexed] voter: &ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] block: u64,
        #[indexed] timestamp: u64,
        event: &VoteEvent<Self::Api>,
    );
}
