use multiversx_sc::imports::*;

use crate::StakedAssetIdentifier;

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getAllowedNftCollections)]
    #[storage_mapper("allowedNftCollections")]
    fn allowed_nft_collections(&self) -> SetMapper<TokenIdentifier>;

    #[view(getRewardTokenIds)]
    #[storage_mapper("rewardTokenIds")]
    fn reward_token_ids(&self) -> SetMapper<TokenIdentifier>;

    #[view(getStakeQuantity)]
    #[storage_mapper("stakeQuantity")]
    fn stake_quantity(
        &self,
        address: &ManagedAddress,
        token_id: &TokenIdentifier,
        nonce: u64,
    ) -> SingleValueMapper<BigUint>;

    #[view(getStakedItemsRaw)]
    #[storage_mapper("stakedItems")]
    fn staked_items(&self, address: &ManagedAddress)
        -> SetMapper<StakedAssetIdentifier<Self::Api>>;

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

    #[view(getStakingDisabled)]
    #[storage_mapper("stakingDisabled")]
    fn staking_disabled(&self) -> SingleValueMapper<bool>;
}
