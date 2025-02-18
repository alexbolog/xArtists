use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait AdminModule:
    crate::storage::StorageModule
    + crate::core_logic::CoreLogic
    + crate::utils::UtilsModule
    + crate::reward::reward_rate::RewardRateModule
    + crate::reward::planned_distribution::PlannedDistributionModule
{
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

    /// I don't recommend using this function as it won't update user's storage.
    /// Its effect as of now is to stop other users from staking NFTs from the given collections.
    #[only_owner]
    #[endpoint(disallowCollections)]
    fn disallow_collections(&self, collections: MultiValueManagedVec<TokenIdentifier>) {
        for collection in collections.into_vec().iter() {
            self.allowed_nft_collections()
                .remove(&collection.clone_value());
        }
    }

    /// Distribute rewards to all stakers.
    /// Expects at least a payment that consists of the total amount of tokens to be distributed.
    /// Used for unscheduled reward distributions (e.g. airdrop, campaigns, module integrations etc).
    #[only_owner]
    #[payable("*")]
    #[endpoint(distributeRewards)]
    fn distribute_rewards(&self) {
        self.require_staking_enabled();

        let payments = self.call_value().all_esdt_transfers();
        self.handle_distribute_rewards(&payments);
    }

    /// Set the unstaking penalty.
    /// A period of time in seconds that users have to wait before they can claim their unstaked NFTs.
    /// Changing this value will affect all users and ongoing unstaking processes.
    #[only_owner]
    #[endpoint(setUnstakingPenalty)]
    fn set_unstaking_penalty(&self, penalty: u64) {
        self.unstaking_penalty().set(penalty);
    }

    /// Change the score for all NFTs in the collection.
    /// Will also add the collection to the list of allowed collections.
    /// This will *NOT* update the score for already staked NFTs.
    #[only_owner]
    #[endpoint(setCollectionScore)]
    fn set_collection_score(&self, collection: TokenIdentifier, score: u64) {
        self.nft_collection_score(&collection)
            .set(BigUint::from(score));
        self.allowed_nft_collections().insert(collection);
    }

    /// Change the score for a specific nonce of an NFT in the collection.
    /// Will also add the collection to the list of allowed collections.
    /// This will *NOT* update the score for already staked NFTs.
    #[only_owner]
    #[endpoint(setCollectionNonceScore)]
    fn set_collection_nonce_score(&self, collection: TokenIdentifier, nonce: u64, score: u64) {
        self.nft_collection_nonce_score(&collection, nonce)
            .set(BigUint::from(score));
        self.allowed_nft_collections().insert(collection);
    }

    /// Create a new distribution plan.
    /// Expects a single payment that consists of the total amount of tokens to be distributed.
    /// The amount per round will be calculated based on the total amount and the number of rounds.
    #[payable("*")]
    #[only_owner]
    #[endpoint(createDistributionPlan)]
    fn create_distribution_plan(&self, start_round: u64, end_round: u64) {
        let payment = self.call_value().single_esdt();
        self.reward_token_ids()
            .insert(payment.token_identifier.clone());
        self.create_plan(
            payment.token_identifier.clone(),
            start_round,
            end_round,
            payment.amount.clone(),
        );
    }

    /// Remove a distribution plan.
    /// Must provide the exact plan configuration to remove.
    #[only_owner]
    #[endpoint(removeDistributionPlan)]
    fn remove_distribution_plan(
        &self,
        reward_token_id: TokenIdentifier,
        start_round: u64,
        end_round: u64,
        amount_per_round: BigUint,
    ) {
        self.remove_plan(reward_token_id, start_round, end_round, amount_per_round);
    }
}
