use multiversx_sc::imports::*;

use crate::constants::ERR_NO_UNSTAKED_ITEMS;

#[multiversx_sc::module]
pub trait CoreLogic:
    crate::storage::StorageModule
    + crate::utils::UtilsModule
    + crate::reward::reward_rate::RewardRateModule
    + crate::reward::planned_distribution::PlannedDistributionModule
{
    fn handle_stake(
        &self,
        user: &ManagedAddress,
        payments: &ManagedVec<EsdtTokenPayment>,
    ) -> BigUint {
        self.handle_state_change(user);

        let mut total_score = BigUint::zero();

        for payment in payments.iter() {
            self.require_can_stake(&payment.token_identifier);
            total_score += self.get_payment_score(&payment);
            self.stake_quantity(user, &payment.token_identifier, payment.token_nonce)
                .update(|prev| *prev += &payment.amount);
            let staked_item = (payment.token_identifier.clone(), payment.token_nonce);
            self.staked_items(user).insert(staked_item);
        }

        self.handle_increase_staked_score(user, &total_score);

        total_score
    }

    fn handle_unstake(
        &self,
        user: &ManagedAddress,
        payments: ManagedVec<EsdtTokenPayment>,
    ) -> BigUint {
        self.handle_state_change(user);

        let mut total_score = BigUint::zero();

        for payment in payments.iter() {
            self.require_user_has_enough_staked_balance(
                user,
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );

            total_score += self.get_payment_score(&payment);
            self.stake_quantity(user, &payment.token_identifier, payment.token_nonce)
                .update(|prev| *prev -= &payment.amount);

            if self
                .stake_quantity(user, &payment.token_identifier, payment.token_nonce)
                .is_empty()
            {
                let staked_item = (payment.token_identifier.clone(), payment.token_nonce);
                self.staked_items(user).remove(&staked_item);
            }
        }

        self.handle_decrease_staked_score(user, &total_score);

        self.unstaking_items(user)
            .insert((self.blockchain().get_block_timestamp(), payments));

        total_score
    }

    fn handle_claim_unstaked(&self, user: &ManagedAddress) {
        let block_timestamp = self.blockchain().get_block_timestamp();
        let unstake_penalty = self.unstaking_penalty().get();
        let mut has_unstaked = false;

        for (unstake_timestamp, payments) in self.unstaking_items(user).iter() {
            let time_passed = block_timestamp - unstake_timestamp;
            if time_passed >= unstake_penalty {
                self.send().direct_multi(user, &payments);
                self.unstaking_items(user)
                    .remove(&(unstake_timestamp, payments));
                has_unstaked = true;
            }
        }

        require!(has_unstaked, ERR_NO_UNSTAKED_ITEMS);
    }

    fn handle_claim_rewards(&self, user: &ManagedAddress) {
        self.handle_state_change(user);
        let mut reward_payments = ManagedVec::new();
        for reward_token_id in self.reward_token_ids().iter() {
            let reward_payment = self.handle_claim_pending_rewards(user, reward_token_id);
            if let Some(reward_payment) = reward_payment {
                reward_payments.push(reward_payment);
            }
        }

        self.send().direct_multi(user, &reward_payments);
    }

    /// This function is called when any user's state changes.
    /// It distributes rewards as planned and stores pending rewards.
    fn handle_state_change(&self, user: &ManagedAddress) {
        self.distribute_as_planned();
        self.handle_store_all_pending_rewards(user);
    }

    fn handle_store_all_pending_rewards(&self, user: &ManagedAddress) {
        for reward_token_id in self.reward_token_ids().iter() {
            self.handle_store_pending_rewards(user, &reward_token_id);
        }
    }

    fn handle_distribute_rewards(&self, rewards: &ManagedVec<EsdtTokenPayment>) {
        for payment in rewards.iter() {
            if !self.reward_token_ids().contains(&payment.token_identifier) {
                self.reward_token_ids()
                    .insert(payment.token_identifier.clone());
            }

            self.handle_increase_reward_rate_from_payment(&payment);
        }
    }
}
