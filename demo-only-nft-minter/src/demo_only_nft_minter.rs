#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod nft_marketplace_proxy;
mod nft_module;

/// An unguarded NFT minter contract to be used for the AI Megawave hackathon to showcase the AI integration.
/// DO NOT USE IN PRODUCTION
#[multiversx_sc::contract]
pub trait DemoOnlyNftMinter: nft_module::NftModule {
   #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[allow_multiple_var_args]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::redundant_closure)]
    #[only_owner]
    #[endpoint(createNft)]
    fn create_nft(
        &self,
        name: ManagedBuffer,
        royalties: BigUint,
        attributes: ManagedBuffer,
        asset_uri: ManagedBuffer,
        metadata_uri: ManagedBuffer,
        selling_price: BigUint,
        opt_token_used_as_payment: OptionalValue<TokenIdentifier>,
        opt_token_used_as_payment_nonce: OptionalValue<u64>,
    ) {
        let token_used_as_payment = match opt_token_used_as_payment {
            OptionalValue::Some(token) => EgldOrEsdtTokenIdentifier::esdt(token),
            OptionalValue::None => EgldOrEsdtTokenIdentifier::egld(),
        };
        require!(
            token_used_as_payment.is_valid(),
            "Invalid token_used_as_payment arg, not a valid token ID"
        );

        let token_used_as_payment_nonce = if token_used_as_payment.is_egld() {
            0
        } else {
            match opt_token_used_as_payment_nonce {
                OptionalValue::Some(nonce) => nonce,
                OptionalValue::None => 0,
            }
        };

        self.create_nft_with_attributes(
            name,
            royalties,
            attributes,
            asset_uri,
            metadata_uri,
            selling_price,
            token_used_as_payment,
            token_used_as_payment_nonce,
        );
    }

    // The marketplace SC will send the funds directly to the initial caller, i.e. the owner
    // The caller has to know which tokens they have to claim,
    // by giving the correct token ID and token nonce
    #[only_owner]
    #[endpoint(claimRoyaltiesFromMarketplace)]
    fn claim_royalties_from_marketplace(
        &self,
        marketplace_address: ManagedAddress,
        token_id: TokenIdentifier,
        token_nonce: u64,
    ) {
        let caller = self.blockchain().get_caller();
        self.tx()
            .to(&marketplace_address)
            .typed(nft_marketplace_proxy::NftMarketplaceProxy)
            .claim_tokens(token_id, token_nonce, caller)
            .async_call_and_exit();
    }
}
