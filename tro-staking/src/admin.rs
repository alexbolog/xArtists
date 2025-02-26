#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait AdminModule: crate::storage::StorageModule {
    #[only_owner]
    #[endpoint(addWhitelistedLpTokens)]
    fn add_whitelisted_lp_tokens(&self, lp_token_identifiers: MultiValueEncoded<TokenIdentifier>) {
        for lp_token_id in lp_token_identifiers {
            self.whitelisted_lp_token_identifiers().insert(lp_token_id);
        }
    }

    #[only_owner]
    #[endpoint(setTroTokenIdentifier)]
    fn set_tro_token_identifier(&self, tro_token_identifier: TokenIdentifier) {
        self.tro_token_identifier().set(tro_token_identifier);
    }
}
