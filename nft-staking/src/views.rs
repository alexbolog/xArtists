use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait ViewsModule:
    crate::storage::StorageModule
    + crate::reward::reward_rate::RewardRateModule
    + crate::reward::planned_distribution::PlannedDistributionModule
{
    #[view(getStakingInfo)]
    fn get_staking_info(&self, address: &ManagedAddress) -> StakingInfo<Self::Api> {
        StakingInfo {
            staked_items: self.get_staked_items_snapshot(address),
            staked_score: self.user_staked_score(address).get(),
            aggregated_staked_score: self.aggregated_staked_score().get(),
            pending_rewards: self.get_pending_rewards_view(address),
            unstaking_items: self.get_unstaking_items(address),
        }
    }

    #[view(getPendingRewards)]
    fn get_pending_rewards_view(
        &self,
        address: &ManagedAddress,
    ) -> ManagedVec<EsdtTokenPayment<Self::Api>> {
        let mut reward_token_ids = ManagedVec::new();
        for reward_token_id in self.reward_token_ids().iter() {
            reward_token_ids.push(reward_token_id.clone());
        }

        let mut rr_rewards = self.get_pending_rewards(address, &reward_token_ids);
        let udr_rewards = self.get_user_undistributed_rewards_share(address);
        rr_rewards.append_vec(udr_rewards);

        rr_rewards
    }

    #[view(getStakedItems)]
    fn get_staked_items_snapshot(
        &self,
        address: &ManagedAddress,
    ) -> ManagedVec<EsdtTokenPayment<Self::Api>> {
        let mut staked_items = ManagedVec::new();

        for (token_id, nonce) in self.staked_items(address).iter() {
            let storage_value = self.stake_quantity(address, &token_id, nonce);
            if storage_value.is_empty() {
                continue;
            }

            staked_items.push(EsdtTokenPayment::new(
                token_id.clone(),
                nonce,
                storage_value.get(),
            ));
        }

        staked_items
    }

    #[view(getUnstakingItems)]
    fn get_unstaking_items(
        &self,
        address: &ManagedAddress,
    ) -> ManagedVec<UnstakingBatch<Self::Api>> {
        let mut unstaking_items = ManagedVec::new();
        for (unstake_timestamp, unstake_items) in self.unstaking_items(address).iter() {
            unstaking_items.push(UnstakingBatch::new(unstake_timestamp, unstake_items));
        }
        unstaking_items
    }

    #[view(getStakeQuantity)]
    fn get_stake_quantity(
        &self,
        address: &ManagedAddress,
        token_id: &TokenIdentifier,
        nonce: u64,
    ) -> BigUint<Self::Api> {
        self.stake_quantity(address, token_id, nonce).get()
    }

    #[view(getUserStakingScore)]
    fn get_user_staking_score(&self, address: &ManagedAddress) -> BigUint<Self::Api> {
        self.user_staked_score(address).get()
    }

    #[view(getAggregatedStakingScore)]
    fn get_aggregated_staking_score(&self) -> BigUint<Self::Api> {
        self.aggregated_staked_score().get()
    }

    #[view(getLastDistributionRound)]
    fn get_last_distribution_round(&self) -> u64 {
        self.last_distribution_round().get()
    }
}

#[derive(TypeAbi, NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct StakingInfo<M: ManagedTypeApi> {
    pub staked_items: ManagedVec<M, EsdtTokenPayment<M>>,
    pub staked_score: BigUint<M>,
    pub aggregated_staked_score: BigUint<M>,
    pub pending_rewards: ManagedVec<M, EsdtTokenPayment<M>>,
    pub unstaking_items: ManagedVec<M, UnstakingBatch<M>>,
}

#[derive(TypeAbi, NestedEncode, NestedDecode, TopEncode, TopDecode, ManagedVecItem)]
pub struct UnstakingBatch<M: ManagedTypeApi> {
    pub unstake_timestamp: u64,
    pub unstake_items: ManagedVec<M, EsdtTokenPayment<M>>,
}

impl<M: ManagedTypeApi> UnstakingBatch<M> {
    pub fn new(unstake_timestamp: u64, unstake_items: ManagedVec<M, EsdtTokenPayment<M>>) -> Self {
        Self {
            unstake_timestamp,
            unstake_items,
        }
    }
}
