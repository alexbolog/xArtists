#[allow(unused_imports)]
use multiversx_sc::imports::*;

use crate::errors::*;

#[multiversx_sc::module]
pub trait StakeModule: crate::storage::StorageModule {
    fn process_stake(&self, user: &ManagedAddress, payments: &ManagedVec<EsdtTokenPayment>) {
        for payment in payments.iter() {
            self.require_token_is_allowed(&payment.token_identifier);
            self.add_payment_to_user_stake(user, &payment);
        }
    }

    fn add_payment_to_user_stake(&self, user: &ManagedAddress, payment: &EsdtTokenPayment) {
        self.users_stake(user, &payment.token_identifier)
            .update(|amount| *amount += &payment.amount);
    }

    fn process_unstake(
        &self,
        user: &ManagedAddress,
        request: MultiValueEncoded<MultiValue2<TokenIdentifier, BigUint>>,
    ) -> ManagedVec<EsdtTokenPayment> {
        let mut payments = ManagedVec::new();

        for request_item in request {
            let (token_identifier, amount) = request_item.into_tuple();
            self.require_token_is_allowed(&token_identifier);
            self.require_user_has_enough_stake(user, &token_identifier, &amount);
            self.subtract_payment_from_user_stake(user, &token_identifier, &amount);

            payments.push(EsdtTokenPayment::new(token_identifier, 0u64, amount));
        }

        payments
    }

    fn subtract_payment_from_user_stake(
        &self,
        user: &ManagedAddress,
        token_identifier: &TokenIdentifier,
        amount: &BigUint,
    ) {
        self.users_stake(user, token_identifier)
            .update(|old_amount| *old_amount -= amount);
    }

    fn require_token_is_allowed(&self, token_identifier: &TokenIdentifier) {
        require!(
            token_identifier == &self.tro_token_identifier().get()
                || self
                    .whitelisted_lp_token_identifiers()
                    .contains(token_identifier),
            ERR_INVALID_PAYMENT_TOKEN
        );
    }

    fn require_user_has_enough_stake(
        &self,
        user: &ManagedAddress,
        token_identifier: &TokenIdentifier,
        amount: &BigUint,
    ) {
        require!(
            &self.users_stake(user, token_identifier).get() >= amount,
            ERR_INSUFFICIENT_STAKE
        );
    }
}
