use crate::constants::*;
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait CoreLogic: crate::storage::StorageModule + crate::utils::UtilsModule {
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
}
