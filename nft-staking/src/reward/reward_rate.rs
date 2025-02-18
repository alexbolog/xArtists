multiversx_sc::imports!();

pub const REWARD_RATE_DENOMINATION: u64 = 1_000_000_000_000_000_000;

#[multiversx_sc::module]
pub trait RewardRateModule {
    fn handle_increase_staked_score(&self, user: &ManagedAddress, amount: &BigUint) {
        self.user_staked_score(user).update(|prev| *prev += amount);
        self.aggregated_staked_score()
            .update(|prev| *prev += amount);
    }

    fn handle_decrease_staked_score(&self, user: &ManagedAddress, amount: &BigUint) {
        self.user_staked_score(user).update(|prev| *prev -= amount);
        self.aggregated_staked_score()
            .update(|prev| *prev -= amount);
    }

    fn handle_increase_reward_rate_from_payment(&self, payment: &EsdtTokenPayment) {
        self.handle_increase_reward_rate_by_denominated_amount(
            &payment.token_identifier,
            &payment.amount,
        );
    }

    fn handle_increase_reward_rate_by_denominated_amount(
        &self,
        token_id: &TokenIdentifier,
        amount: &BigUint,
    ) {
        let aggregated_stake_score = self.aggregated_staked_score().get();
        let distribution_rate_increase = amount / &aggregated_stake_score;

        self.current_reward_rate(token_id)
            .update(|prev| *prev += &distribution_rate_increase);
    }

    fn handle_store_pending_rewards(
        &self,
        user: &ManagedAddress,
        reward_token_id: &TokenIdentifier,
    ) {
        let rewards = self.get_unstored_rewards_for_token(user, reward_token_id);

        if rewards == 0 {
            return;
        }

        self.user_stored_rewards(user, reward_token_id)
            .update(|prev| *prev += &rewards);
        self.user_reward_rate(user, reward_token_id)
            .set(self.current_reward_rate(reward_token_id).get());
    }

    fn get_unstored_rewards_for_token(
        &self,
        user: &ManagedAddress,
        reward_token_id: &TokenIdentifier,
    ) -> BigUint {
        let current_rate = self.current_reward_rate(reward_token_id).get();
        let user_rate = match self.user_reward_rate(user, reward_token_id).is_empty() {
            true => current_rate.clone(),
            false => self.user_reward_rate(user, reward_token_id).get(),
        };

        let rate_diff = &current_rate - &user_rate;
        if rate_diff == 0 {
            return BigUint::zero();
        }

        rate_diff * self.user_staked_score(user).get() / REWARD_RATE_DENOMINATION
    }

    fn handle_claim_pending_rewards(
        &self,
        user: &ManagedAddress,
        reward_token_id: TokenIdentifier,
    ) -> Option<EsdtTokenPayment> {
        self.handle_store_pending_rewards(user, &reward_token_id);

        let rewards = self.user_stored_rewards(user, &reward_token_id).get();
        if rewards == 0 {
            return None;
        }

        self.user_stored_rewards(user, &reward_token_id).clear();

        Some(EsdtTokenPayment::new(reward_token_id, 0, rewards))
    }

    fn get_pending_rewards(
        &self,
        user: &ManagedAddress,
        reward_token_ids: &ManagedVec<TokenIdentifier>,
    ) -> ManagedVec<EsdtTokenPayment<Self::Api>> {
        let mut pending_rewards = ManagedVec::new();
        for reward_token_id in reward_token_ids.iter() {
            let reward_payment =
                self.get_pending_rewards_for_token(user, reward_token_id.clone_value());
            if let Some(reward_payment) = reward_payment {
                pending_rewards.push(reward_payment);
            }
        }
        pending_rewards
    }

    fn get_pending_rewards_for_token(
        &self,
        user: &ManagedAddress,
        reward_token_id: TokenIdentifier,
    ) -> Option<EsdtTokenPayment<Self::Api>> {
        let stored_rewards = self.user_stored_rewards(user, &reward_token_id).get();
        let unstored_rewards = self.get_unstored_rewards_for_token(user, &reward_token_id);

        let total_rewards = stored_rewards + unstored_rewards;

        if total_rewards == 0 {
            return None;
        }

        Some(EsdtTokenPayment::new(reward_token_id, 0, total_rewards))
    }

    #[view(getUserStakedScore)]
    #[storage_mapper("userStakedScore")]
    fn user_staked_score(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getAggregatedStakedScore)]
    #[storage_mapper("aggregatedStakedScore")]
    fn aggregated_staked_score(&self) -> SingleValueMapper<BigUint>;

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
}
