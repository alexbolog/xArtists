use crate::constants::*;
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait UtilsModule: crate::storage::StorageModule {
    fn get_payment_score(&self, payment: &EsdtTokenPayment) -> BigUint {
        let score = self.get_nft_score(&payment.token_identifier, payment.token_nonce);

        &score * &payment.amount
    }

    fn get_nft_score(&self, token_id: &TokenIdentifier, nonce: u64) -> BigUint {
        if !self.nft_collection_nonce_score(token_id, nonce).is_empty() {
            return self.nft_collection_nonce_score(token_id, nonce).get();
        }

        if !self.nft_collection_score(token_id).is_empty() {
            return self.nft_collection_score(token_id).get();
        }

        BigUint::from(DEFAULT_NFT_SCORE)
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
            &self.stake_quantity(user, token_id, nonce).get() >= amount,
            ERR_USER_HAS_NOT_ENOUGH_STAKED_BALANCE
        );
    }
}
