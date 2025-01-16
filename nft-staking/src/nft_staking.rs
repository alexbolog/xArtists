#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

// errors
pub const ERR_NFT_COLLECTION_NOT_ALLOWED: &str = "NFT collection not allowed";
pub const ERR_USER_HAS_NOT_ENOUGH_STAKED_BALANCE: &str = "User has not enough staked balance";
pub const ERR_STAKING_DISABLED: &str = "Staking is disabled";
pub const REWARD_RATE_DENOMINATION: u64 = 1_000_000_000_000_000_000;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait NftStaking {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        let caller = self.blockchain().get_caller();
        self.require_staking_enabled();

        let payments = self.call_value().all_esdt_transfers();
        self.handle_stake(&caller, &payments);
    }

    #[endpoint(unstake)]
    fn unstake(&self, unstake_request: MultiValueManagedVec<EsdtTokenPayment>) {
        let caller = self.blockchain().get_caller();
        self.require_staking_enabled();

        let payments = unstake_request.into_vec();
        self.handle_unstake(&caller, &payments);

        self.send().direct_multi(&caller, &payments);
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        self.require_staking_enabled();
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(distributeRewards)]
    fn distribute_rewards(&self) {
        self.require_staking_enabled();
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

    // core
    fn handle_stake(
        &self,
        user: &ManagedAddress,
        payments: &ManagedVec<EsdtTokenPayment>,
    ) -> BigUint {
        self.handle_store_pending_rewards(user);

        let mut total_score = BigUint::zero();

        for payment in payments.iter() {
            self.require_can_stake(&payment.token_identifier);
            total_score += self.get_payment_score(&payment);
            self.stake_info(user, &payment.token_identifier, payment.token_nonce)
                .update(|prev| *prev += &payment.amount);
        }

        self.user_staked_score(user)
            .update(|prev| *prev += &total_score);
        self.aggregated_staked_score()
            .update(|prev| *prev += &total_score);

        total_score
    }

    fn handle_unstake(
        &self,
        user: &ManagedAddress,
        payments: &ManagedVec<EsdtTokenPayment>,
    ) -> BigUint {
        self.handle_store_pending_rewards(user);

        let mut total_score = BigUint::zero();

        for payment in payments.iter() {
            self.require_user_has_enough_staked_balance(
                user,
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );

            total_score += self.get_payment_score(&payment);
            self.stake_info(user, &payment.token_identifier, payment.token_nonce)
                .update(|prev| *prev -= &payment.amount);
        }

        self.user_staked_score(user)
            .update(|prev| *prev -= &total_score);
        self.aggregated_staked_score()
            .update(|prev| *prev -= &total_score);

        total_score
    }

    fn handle_claim_rewards(&self, user: &ManagedAddress) {
        self.handle_store_pending_rewards(user);

        for reward_token_id in self.reward_token_ids().iter() {
            let rewards = self.user_stored_rewards(user, &reward_token_id).get();
            if rewards == 0 {
                continue;
            }

            self.user_stored_rewards(user, &reward_token_id).clear();

            self.send().direct_esdt(user, &reward_token_id, 0, &rewards);
        }
    }

    fn handle_store_pending_rewards(&self, user: &ManagedAddress) {
        for reward_token_id in self.reward_token_ids().iter() {
            let current_rate = self.current_reward_rate(&reward_token_id).get();
            let user_rate = match self.user_reward_rate(user, &reward_token_id).is_empty() {
                true => BigUint::zero(), // TODO: check if this is correct, maybe should it be one?
                false => self.user_reward_rate(user, &reward_token_id).get(),
            };

            let rate_diff = &current_rate - &user_rate;
            if rate_diff == 0 {
                continue;
            }

            let rewards = rate_diff * self.user_staked_score(user).get() / REWARD_RATE_DENOMINATION;

            self.user_stored_rewards(user, &reward_token_id)
                .update(|prev| *prev += &rewards);
            self.user_reward_rate(user, &reward_token_id)
                .set(current_rate);
        }
    }

    fn handle_distribute_rewards(&self, rewards: &ManagedVec<EsdtTokenPayment>) {
        for payment in rewards.iter() {
            if !self.reward_token_ids().contains(&payment.token_identifier) {
                self.reward_token_ids()
                    .insert(payment.token_identifier.clone());
            }

            let aggregated_stake_score = self.aggregated_staked_score().get();
            let distribution_rate_increase =
                &payment.amount * REWARD_RATE_DENOMINATION / &aggregated_stake_score;

            self.current_reward_rate(&payment.token_identifier)
                .update(|prev| *prev += &distribution_rate_increase);
        }
    }

    // utils
    fn get_payment_score(&self, payment: &EsdtTokenPayment) -> BigUint {
        let score = self.get_nft_score(&payment.token_identifier, payment.token_nonce);

        &score * &payment.amount
    }

    fn get_nft_score(&self, token_id: &TokenIdentifier, nonce: u64) -> BigUint {
        if self.nft_collection_nonce_score(token_id, nonce).is_empty() {
            return self.nft_collection_score(token_id).get();
        }

        self.nft_collection_nonce_score(token_id, nonce).get()
    }

    fn require_staking_enabled(&self) {
        require!(!self.staking_disabled().get(), ERR_STAKING_DISABLED);
    }

    fn require_can_stake(&self, token_id: &TokenIdentifier) {
        require!(
            self.allowed_nft_collections().contains(token_id),
            ERR_NFT_COLLECTION_NOT_ALLOWED
        );
    }

    fn require_user_has_enough_staked_balance(
        &self,
        user: &ManagedAddress,
        token_id: &TokenIdentifier,
        nonce: u64,
        amount: &BigUint,
    ) {
        require!(
            &self.stake_info(user, token_id, nonce).get() >= amount,
            ERR_USER_HAS_NOT_ENOUGH_STAKED_BALANCE
        );
    }

    // Storage
    #[view(getAllowedNftCollections)]
    #[storage_mapper("allowedNftCollections")]
    fn allowed_nft_collections(&self) -> SetMapper<TokenIdentifier>;

    #[view(getStakeInfo)]
    #[storage_mapper("stakeInfo")]
    fn stake_info(
        &self,
        address: &ManagedAddress,
        token_id: &TokenIdentifier,
        nonce: u64,
    ) -> SingleValueMapper<BigUint>;

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
