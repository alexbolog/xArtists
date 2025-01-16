#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getTroTokenIdentifier)]
    #[storage_mapper("tro_token_identifier")]
    fn tro_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getWhitelistedLpTokenIdentifiers)]
    #[storage_mapper("whitelisted_lp_token_identifiers")]
    fn whitelisted_lp_token_identifiers(&self) -> SetMapper<TokenIdentifier>;

    #[view(getUsersStake)]
    #[storage_mapper("users_stake")]
    fn users_stake(
        &self,
        users_address: &ManagedAddress,
        token_identifier: &TokenIdentifier,
    ) -> SingleValueMapper<BigUint>;
}
