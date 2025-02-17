use super::reward_rate::REWARD_RATE_DENOMINATION;

multiversx_sc::imports!();

pub type DistributionPlan<M> = (TokenIdentifier<M>, u64, u64, BigUint<M>); // token, start round, end round, denominated amount (*REWARD_RATE_DENOMINATION)

#[multiversx_sc::module]
pub trait PlannedDistributionModule: super::reward_rate::RewardRateModule {
    fn create_plan(
        &self,
        token: TokenIdentifier,
        start_round: u64,
        end_round: u64,
        total_distribution_amount: BigUint,
    ) {
        let amount_per_round =
            self.get_amount_per_round(start_round, end_round, total_distribution_amount);
        self.distribution_plans()
            .insert((token, start_round, end_round, amount_per_round));
    }

    #[view(getDistributionAmountPerRound)]
    fn get_amount_per_round(
        &self,
        start_round: u64,
        end_round: u64,
        total_distribution_amount: BigUint,
    ) -> BigUint {
        total_distribution_amount * REWARD_RATE_DENOMINATION / (end_round - start_round)
    }

    fn remove_plan(
        &self,
        token: TokenIdentifier,
        start_round: u64,
        end_round: u64,
        amount_per_round: BigUint,
    ) {
        let plan = (token, start_round, end_round, amount_per_round);

        if let Some(amount) =
            self.get_amount_to_distribute(&plan, self.blockchain().get_block_round())
        {
            self.handle_increase_reward_rate_by_denominated_amount(&plan.0, &amount);
        }

        require!(self.distribution_plans().remove(&plan), "Plan not found");
        self.last_distribution_round().clear();
    }

    fn distribute_as_planned(&self) {
        let current_round = self.blockchain().get_block_round();
        for plan in self.distribution_plans().iter() {
            if let Some(amount) = self.get_amount_to_distribute(&plan, current_round) {
                self.handle_increase_reward_rate_by_denominated_amount(&plan.0, &amount);
                self.last_distribution_round().set(current_round);
            } else {
                self.distribution_plans().remove(&plan);
            }
        }
    }

    /// Will return None if the plan is no longer active
    fn get_amount_to_distribute(
        &self,
        plan: &DistributionPlan<Self::Api>,
        current_round: u64,
    ) -> Option<BigUint<Self::Api>> {
        let (_, start_round, end_round, amount_per_round) = plan;

        if current_round > *end_round {
            return None;
        }

        if current_round < *start_round {
            return Some(BigUint::zero());
        }

        let amount = amount_per_round * (current_round - start_round);
        Some(amount)
    }

    fn get_all_planned_undistributed_rewards(&self) -> ManagedVec<EsdtTokenPayment<Self::Api>> {
        let mut rewards = ManagedVec::new();
        for plan in self.distribution_plans().iter() {
            if let Some(amount) =
                self.get_amount_to_distribute(&plan, self.blockchain().get_block_round())
            {
                rewards.push(EsdtTokenPayment::new(plan.0, 0, amount));
            }
        }
        rewards
    }

    fn get_user_undistributed_rewards_share(
        &self,
        user: &ManagedAddress,
    ) -> ManagedVec<EsdtTokenPayment<Self::Api>> {
        let mut rewards = ManagedVec::new();

        let undistributed_rewards = self.get_all_planned_undistributed_rewards();
        if undistributed_rewards.is_empty() {
            return rewards;
        }

        let total_score = self.user_staked_score(user).get();
        let user_score = self.user_staked_score(user).get();

        for reward in undistributed_rewards.iter() {
            let reward_payment = self.get_pending_rewards_for_token(user, reward.token_identifier);
            if let Some(mut reward_payment) = reward_payment {
                reward_payment.amount = reward_payment.amount * &user_score / &total_score;
                rewards.push(reward_payment);
            }
        }

        rewards
    }

    #[view(getLastDistributionRoundRaw)]
    #[storage_mapper("lastDistributionRound")]
    fn last_distribution_round(&self) -> SingleValueMapper<u64>;

    #[view(getDistributionPlanRaw)]
    #[storage_mapper("distributionPlans")]
    fn distribution_plans(&self) -> SetMapper<DistributionPlan<Self::Api>>;
}
