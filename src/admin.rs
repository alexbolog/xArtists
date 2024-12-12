#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait AdminModule: crate::storage::StorageModule {
    #[endpoint(addWhitelistedLpTokens)]
    fn add_whitelisted_lp_tokens(&self, lp_token_identifiers: MultiValueEncoded<TokenIdentifier>) {
        for lp_token_id in lp_token_identifiers {
            self.whitelisted_lp_token_identifiers().insert(lp_token_id);
        }
    }
}
