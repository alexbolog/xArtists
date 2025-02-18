#![no_std]

use constants::UNSTAKE_PENALTY;
#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod admin;
pub mod constants;
pub mod core_logic;
pub mod proxy;
pub mod reward;
pub mod storage;
pub mod utils;
pub mod views;

pub type StakedAssetIdentifier<M> = (TokenIdentifier<M>, u64);

#[multiversx_sc::contract]
pub trait NftStaking:
    storage::StorageModule
    + core_logic::CoreLogic
    + utils::UtilsModule
    + views::ViewsModule
    + reward::reward_rate::RewardRateModule
    + reward::planned_distribution::PlannedDistributionModule
    + admin::AdminModule
{
    #[init]
    fn init(&self) {
        self.set_unstaking_penalty(UNSTAKE_PENALTY);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) -> BigUint {
        self.require_staking_enabled();

        let caller = self.blockchain().get_caller();
        let payments = self.call_value().all_esdt_transfers();

        self.handle_stake(&caller, &payments)
    }

    #[endpoint(unstake)]
    fn unstake(&self, unstake_request: MultiValueManagedVec<EsdtTokenPayment>) -> BigUint {
        self.require_staking_enabled();

        let caller = self.blockchain().get_caller();
        let payments = unstake_request.into_vec();

        self.handle_unstake(&caller, payments)
    }

    #[endpoint(claimUnstaked)]
    fn claim_unstaked(&self) {
        self.require_staking_enabled();

        let caller = self.blockchain().get_caller();
        self.handle_claim_unstaked(&caller);
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        self.require_staking_enabled();

        let caller = self.blockchain().get_caller();
        self.handle_claim_rewards(&caller);
    }
}
