use multiversx_sc::imports::*;

use crate::StakedAssetIdentifier;

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getAllowedNftCollections)]
    #[storage_mapper("allowedNftCollections")]
    fn allowed_nft_collections(&self) -> SetMapper<TokenIdentifier>;

    #[view(getStakeQuantity)]
    #[storage_mapper("stakeQuantity")]
    fn stake_quantity(
        &self,
        address: &ManagedAddress,
        token_id: &TokenIdentifier,
        nonce: u64,
    ) -> SingleValueMapper<BigUint>;

    #[view(getStakedItems)]
    #[storage_mapper("stakedItems")]
    fn staked_items(&self, address: &ManagedAddress)
        -> SetMapper<StakedAssetIdentifier<Self::Api>>;

    #[view(getUserStakedScore)]
    #[storage_mapper("userStakedScore")]
    fn user_staked_score(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getAggregatedStakedScore)]
    #[storage_mapper("aggregatedStakedScore")]
    fn aggregated_staked_score(&self) -> SingleValueMapper<BigUint>;

    #[view(getRewardRate)]
    #[storage_mapper("rewardRate")]
    fn reward_rate(&self, reward_token_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getRewardTokenIds)]
    #[storage_mapper("rewardTokenIds")]
    fn reward_token_ids(&self) -> SetMapper<TokenIdentifier>;

    #[view(getNftCollectionScore)]
    #[storage_mapper("nftCollectionScore")]
    fn nft_collection_score(&self, token_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getNftCollectionNonceScore)]
    #[storage_mapper("nftCollectionNonceScore")]
    fn nft_collection_nonce_score(
        &self,
        token_id: &TokenIdentifier,
        nonce: u64,
    ) -> SingleValueMapper<BigUint>;

    #[view(getCurrentRewardRate)]
    #[storage_mapper("currentRewardRate")]
    fn current_reward_rate(&self, reward_token_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getUserRewardRate)]
    #[storage_mapper("userRewardRate")]
    fn user_reward_rate(
        &self,
        user: &ManagedAddress,
        reward_token_id: &TokenIdentifier,
    ) -> SingleValueMapper<BigUint>;

    #[view(getUserStoredRewards)]
    #[storage_mapper("userStoredRewards")]
    fn user_stored_rewards(
        &self,
        user: &ManagedAddress,
        reward_token_id: &TokenIdentifier,
    ) -> SingleValueMapper<BigUint>;

    #[view(getStakingDisabled)]
    #[storage_mapper("stakingDisabled")]
    fn staking_disabled(&self) -> SingleValueMapper<bool>;
}
