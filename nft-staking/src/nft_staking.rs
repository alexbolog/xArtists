#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod constants;
pub mod core_logic;
pub mod storage;
pub mod utils;

pub type StakedAssetIdentifier<M> = (TokenIdentifier<M>, u64);

#[multiversx_sc::contract]
pub trait NftStaking: storage::StorageModule + core_logic::CoreLogic + utils::UtilsModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        self.require_staking_enabled();

        let caller = self.blockchain().get_caller();
        let payments = self.call_value().all_esdt_transfers();

        self.handle_stake(&caller, &payments);
    }

    #[endpoint(unstake)]
    fn unstake(&self, unstake_request: MultiValueManagedVec<EsdtTokenPayment>) {
        self.require_staking_enabled();

        let caller = self.blockchain().get_caller();
        let payments = unstake_request.into_vec();

        self.handle_unstake(&caller, &payments);

        self.send().direct_multi(&caller, &payments);
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        self.require_staking_enabled();

        let caller = self.blockchain().get_caller();
        self.handle_claim_rewards(&caller);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(distributeRewards)]
    fn distribute_rewards(&self) {
        self.require_staking_enabled();

        let payments = self.call_value().all_esdt_transfers();
        self.handle_distribute_rewards(&payments);
    }

    #[only_owner]
    #[endpoint(disableStaking)]
    fn disable_staking(&self) {
        self.staking_disabled().set(true);
    }

    #[only_owner]
    #[endpoint(enableStaking)]
    fn enable_staking(&self) {
        self.staking_disabled().set(false);
    }

    #[only_owner]
    #[endpoint(allowCollections)]
    fn allow_collections(&self, collections: MultiValueManagedVec<TokenIdentifier>) {
        for collection in collections.into_vec().iter() {
            self.allowed_nft_collections()
                .insert(collection.clone_value());
        }
    }

    #[only_owner]
    #[endpoint(disallowCollections)]
    fn disallow_collections(&self, collections: MultiValueManagedVec<TokenIdentifier>) {
        for collection in collections.into_vec().iter() {
            self.allowed_nft_collections()
                .remove(&collection.clone_value());
        }
    }
}
