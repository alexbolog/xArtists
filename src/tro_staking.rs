#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod proxy;

mod admin;
mod errors;
mod stake;
mod storage;

/// $TRO staking smart contract
/// Users can stake $TRO and LP tokens in order to:
/// - participate in the xArtist governance mechanism
/// - earn rewards? TODO: check with team
#[multiversx_sc::contract]
pub trait TroStaking: storage::StorageModule + stake::StakeModule + admin::AdminModule {
    #[init]
    fn init(
        &self,
        tro_token_identifier: TokenIdentifier,
        lp_token_identifiers: MultiValueEncoded<TokenIdentifier>,
    ) {
        self.tro_token_identifier().set(tro_token_identifier);
        self.add_whitelisted_lp_tokens(lp_token_identifiers);
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        let caller = self.blockchain().get_caller();
        let payments = self.call_value().all_esdt_transfers();

        self.process_stake(&caller, &payments);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
