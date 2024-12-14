// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct TroStakingProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for TroStakingProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = TroStakingProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        TroStakingProxyMethods { wrapped_tx: tx }
    }
}

pub struct TroStakingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> TroStakingProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, TokenIdentifier<Env::Api>>>,
    >(
        self,
        tro_token_identifier: Arg0,
        lp_token_identifiers: Arg1,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&tro_token_identifier)
            .argument(&lp_token_identifiers)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> TroStakingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> TroStakingProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn stake(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("stake")
            .original_result()
    }

    pub fn unstake<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, MultiValue2<TokenIdentifier<Env::Api>, BigUint<Env::Api>>>>,
    >(
        self,
        request: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unstake")
            .argument(&request)
            .original_result()
    }

    pub fn tro_token_identifier(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTroTokenIdentifier")
            .original_result()
    }

    pub fn whitelisted_lp_token_identifiers(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, TokenIdentifier<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getWhitelistedLpTokenIdentifiers")
            .original_result()
    }

    pub fn users_stake<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        users_address: Arg0,
        token_identifier: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUsersStake")
            .argument(&users_address)
            .argument(&token_identifier)
            .original_result()
    }

    pub fn add_whitelisted_lp_tokens<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, TokenIdentifier<Env::Api>>>,
    >(
        self,
        lp_token_identifiers: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addWhitelistedLpTokens")
            .argument(&lp_token_identifiers)
            .original_result()
    }

    pub fn create_proposal<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
        Arg3: ProxyArg<OptionalValue<u64>>,
        Arg4: ProxyArg<OptionalValue<u64>>,
        Arg5: ProxyArg<MultiValueEncoded<Env::Api, MultiValue3<TokenIdentifier<Env::Api>, BigUint<Env::Api>, BigUint<Env::Api>>>>,
    >(
        self,
        title: Arg0,
        description: Arg1,
        min_voting_power_to_validate_vote: Arg2,
        start_time: Arg3,
        end_time: Arg4,
        lp_to_tro_ratios: Arg5,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("createProposal")
            .argument(&title)
            .argument(&description)
            .argument(&min_voting_power_to_validate_vote)
            .argument(&start_time)
            .argument(&end_time)
            .argument(&lp_to_tro_ratios)
            .original_result()
    }

    pub fn vote<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<VoteDecision>,
    >(
        self,
        proposal_id: Arg0,
        decision: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("vote")
            .argument(&proposal_id)
            .argument(&decision)
            .original_result()
    }

    pub fn last_proposal_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLastProposalId")
            .original_result()
    }

    pub fn proposals<
        Arg0: ProxyArg<u64>,
    >(
        self,
        proposal_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Proposal<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getProposal")
            .argument(&proposal_id)
            .original_result()
    }

    pub fn proposal_votes<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<VoteDecision>,
    >(
        self,
        proposal_id: Arg0,
        decision: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getProposalVotes")
            .argument(&proposal_id)
            .argument(&decision)
            .original_result()
    }

    pub fn user_votes<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        user: Arg0,
        proposal_id: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, VoteContext<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUserVote")
            .argument(&user)
            .argument(&proposal_id)
            .original_result()
    }

    pub fn lp_to_tro_ratio<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        proposal_id: Arg0,
        lp_token: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLpToTroRatio")
            .argument(&proposal_id)
            .argument(&lp_token)
            .original_result()
    }

    pub fn get_voting_power_view<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<OptionalValue<u64>>,
    >(
        self,
        user: Arg0,
        proposal_id: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getVotingPower")
            .argument(&user)
            .argument(&proposal_id)
            .original_result()
    }

    pub fn get_staking_context<
        Arg0: ProxyArg<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        user: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OptionalValue<StakingContext<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getStakingContext")
            .argument(&user)
            .original_result()
    }

    pub fn get_user_complete_stake<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        user: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUserCompleteStake")
            .argument(&user)
            .original_result()
    }

    pub fn get_user_stake<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        user: Arg0,
        token_identifier: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, EsdtTokenPayment<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUserStake")
            .argument(&user)
            .argument(&token_identifier)
            .original_result()
    }

    pub fn get_proposal_context<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        proposal_id: Arg0,
        user: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ProposalContext<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getProposalContext")
            .argument(&proposal_id)
            .argument(&user)
            .original_result()
    }

    pub fn get_active_proposal_ids(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, u64>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getActiveProposalIds")
            .original_result()
    }

    pub fn get_proposal_status_view<
        Arg0: ProxyArg<u64>,
    >(
        self,
        proposal_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ProposalStatus> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getProposalStatus")
            .argument(&proposal_id)
            .original_result()
    }

    pub fn get_proposal_vote_context_view<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        proposal_id: Arg0,
        voter: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OptionalValue<VoteContext<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getProposalVoteContext")
            .argument(&proposal_id)
            .argument(&voter)
            .original_result()
    }

    pub fn get_proposal_vote_count_view<
        Arg0: ProxyArg<u64>,
    >(
        self,
        proposal_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ProposalVoteCount<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getProposalVoteCount")
            .argument(&proposal_id)
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub enum VoteDecision {
    Invalid,
    Approve,
    Abstain,
    Reject,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct Proposal<Api>
where
    Api: ManagedTypeApi,
{
    pub id: u64,
    pub title: ManagedBuffer<Api>,
    pub description: ManagedBuffer<Api>,
    pub creator: ManagedAddress<Api>,
    pub created_at: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub min_voting_power_to_validate_vote: BigUint<Api>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct VoteContext<Api>
where
    Api: ManagedTypeApi,
{
    pub decision: VoteDecision,
    pub voting_power: BigUint<Api>,
    pub timestamp: u64,
    pub block: u64,
    pub epoch: u64,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct StakeEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub payments: ManagedVec<Api, EsdtTokenPayment<Api>>,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct ProposalCreatedEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub creator: ManagedAddress<Api>,
    pub proposal_id: u64,
    pub title: ManagedBuffer<Api>,
    pub min_voting_power: BigUint<Api>,
    pub start_time: u64,
    pub end_time: u64,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct StakingContext<Api>
where
    Api: ManagedTypeApi,
{
    pub users_stake: ManagedVec<Api, EsdtTokenPayment<Api>>,
    pub last_proposals_context: ProposalContext<Api>,
    pub active_proposal_ids: ManagedVec<Api, u64>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct ProposalContext<Api>
where
    Api: ManagedTypeApi,
{
    pub proposal: Proposal<Api>,
    pub users_voting_power: BigUint<Api>,
    pub users_vote: Option<VoteContext<Api>>,
    pub proposal_status: ProposalStatus,
    pub proposal_vote_count: ProposalVoteCount<Api>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub enum ProposalStatus {
    Invalid,
    Pending,
    Active,
    Approved,
    Rejected,
    Failed,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct ProposalVoteCount<Api>
where
    Api: ManagedTypeApi,
{
    pub approve: BigUint<Api>,
    pub abstain: BigUint<Api>,
    pub reject: BigUint<Api>,
    pub invalid: BigUint<Api>,
}
