#[allow(unused_imports)]
use multiversx_sc::imports::*;

use crate::errors::ERR_INVALID_PAYMENT_TOKEN;

#[multiversx_sc::module]
pub trait StakeModule: crate::storage::StorageModule {
    fn process_stake(&self, user: &ManagedAddress, payments: &ManagedVec<EsdtTokenPayment>) {
        for payment in payments.iter() {
            self.require_payment_token_is_allowed(&payment);
            self.add_payment_to_user_stake(user, &payment);
        }
    }

    fn require_payment_token_is_allowed(&self, payment: &EsdtTokenPayment) {
        require!(
            payment.token_identifier == self.tro_token_identifier().get()
                || self
                    .whitelisted_lp_token_identifiers()
                    .contains(&payment.token_identifier),
            ERR_INVALID_PAYMENT_TOKEN
        );
    }

    fn add_payment_to_user_stake(&self, user: &ManagedAddress, payment: &EsdtTokenPayment) {
        self.users_stake(user, &payment.token_identifier)
            .update(|amount| *amount += &payment.amount);
    }
}
