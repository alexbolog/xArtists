#![no_std]

use multiversx_sc::derive_imports::*;
#[allow(unused_imports)]
use multiversx_sc::imports::*;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait DemoOnlyEscrow {
    #[init]
    fn init(&self, demo_collection: TokenIdentifier) {
        self.demo_collection().set(demo_collection);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(lock)]
    fn lock_nft(&self) {
        let caller = self.blockchain().get_caller();
        let payments = self.call_value().all_esdt_transfers();

        let demo_collection = self.demo_collection().get();

        for payment in payments.iter() {
            require!(payment.token_identifier == demo_collection, "Invalid token");

            self.locked_nfts(payment.token_nonce)
                .set((caller.clone(), false)); // not yet updated by escrow account
            self.user_demo_loaded_nonces(&caller)
                .insert(payment.token_nonce);
        }
    }

    #[only_owner]
    #[endpoint(update)]
    fn update(
        &self,
        nonce: u64,
        name: ManagedBuffer,
        royalties: u64,
        new_attributes: ManagedBuffer,
        artwork_uri: ManagedBuffer,
        // metadata_uri: ManagedBuffer,
    ) {
        let token_identifier = self.demo_collection().get();

        // let mut uris = ManagedVec::new();
        // uris.push(artwork_uri);
        // // uris.push(metadata_uri);

        // let attributes_sha256 = self.crypto().sha256(&new_attributes);
        // let hash = attributes_sha256.as_managed_buffer();

        // self.send().esdt_metadata_recreate(
        //     token_identifier,
        //     nonce,
        //     name,
        //     royalties,
        //     hash.clone(),
        //     &new_attributes,
        //     uris,
        // );

        // self.locked_nfts(nonce).update(|prev| {
        //     *prev = (prev.0.clone(), true);
        // });

        self.send()
            .nft_update_attributes(&token_identifier, nonce, &new_attributes);
    }

    #[endpoint(unlock)]
    fn unlock(&self, nonce: u64) {
        let caller = self.blockchain().get_caller();

        let (user, can_be_claimed) = self.locked_nfts(nonce).get();
        require!(user == caller, "Not authorized");
        require!(can_be_claimed, "Can't claim yet");

        self.locked_nfts(nonce).clear();
        self.user_demo_loaded_nonces(&caller).remove(&nonce);

        self.send().direct_esdt(
            &caller,
            &self.demo_collection().get(),
            nonce,
            &BigUint::from(1u32),
        );
    }

    #[view(getStatus)]
    fn get_status(&self, user: ManagedAddress) -> UserStatus<Self::Api> {
        let mut ready_nonces = ManagedVec::new();
        let mut locked_nonces = ManagedVec::new();

        for nonce in self.user_demo_loaded_nonces(&user).iter() {
            let (_, is_unlocked) = self.locked_nfts(nonce).get();
            if is_unlocked {
                ready_nonces.push(nonce);
            } else {
                locked_nonces.push(nonce);
            }
        }

        UserStatus {
            ready_nonces,
            user_address: user,
            locked_nonces,
        }
    }

    #[view(getDemoCollection)]
    #[storage_mapper("demo_collection")]
    fn demo_collection(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getLockedNft)]
    #[storage_mapper("locked_nfts")]
    fn locked_nfts(&self, nonce: u64) -> SingleValueMapper<(ManagedAddress, bool)>; // (user, has been updated by escrow)

    #[view(getAvailableUserNonces)]
    #[storage_mapper("user_demo_loaded_nonces")]
    fn user_demo_loaded_nonces(&self, user: &ManagedAddress) -> SetMapper<u64>;
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct UserStatus<M: ManagedTypeApi> {
    pub ready_nonces: ManagedVec<M, u64>,
    pub user_address: ManagedAddress<M>,
    pub locked_nonces: ManagedVec<M, u64>,
}
