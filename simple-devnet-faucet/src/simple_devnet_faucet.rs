#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

const ONE_TOKEN_DENOMINATION: u64 = 1000000000000000000;
const ESDT_TOKEN_AMOUNT: u64 = 5000;

const MIN_CLAIM_INTERVAL_SECONDS: u64 = 60;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait SimpleDevnetFaucet {
    #[init]
    fn init(
        &self,
        esdt_token_identifier: TokenIdentifier,
        sft_token_identifier: TokenIdentifier,
        nft_token_identifier: TokenIdentifier,
    ) {
        self.esdt_token_identifier().set(esdt_token_identifier);
        self.sft_token_identifier().set(sft_token_identifier);
        self.nft_token_identifier().set(nft_token_identifier);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint]
    fn deposit(&self) {
        let deposits = self.call_value().all_esdt_transfers();

        for deposit in deposits.iter() {
            if deposit.token_identifier == self.esdt_token_identifier().get() {
                self.esdt_balance()
                    .update(|previous_balance| *previous_balance += deposit.amount);
                continue;
            }
            if deposit.token_identifier == self.sft_token_identifier().get() {
                self.sft_balance()
                    .entry(deposit.token_nonce)
                    .or_insert(BigUint::zero())
                    .update(|previous_balance| *previous_balance += deposit.amount);
                continue;
            }
            if deposit.token_identifier == self.nft_token_identifier().get() {
                self.nft_balance().push(&deposit.token_nonce);
                continue;
            }

            sc_panic!("Unsupported token identifier");
        }
    }

    #[endpoint]
    fn claim(&self) {
        let caller = self.blockchain().get_caller();

        if self.last_claim(&caller).get() + MIN_CLAIM_INTERVAL_SECONDS
            > self.blockchain().get_block_timestamp()
        {
            sc_panic!("Too soon to claim again");
        }

        let esdt_payment =
            self.claim_esdt(BigUint::from(ESDT_TOKEN_AMOUNT) * ONE_TOKEN_DENOMINATION);
        // let sft_payments = self.claim_sft();
        // let nft_payment = self.claim_nft();

        let mut payments = ManagedVec::new();

        payments.push(esdt_payment);
        // payments.append_vec(sft_payments);
        // payments.push(nft_payment);

        self.send().direct_multi(&caller, &payments);
        self.last_claim(&caller).set(self.blockchain().get_block_timestamp());
    }

    fn claim_esdt(&self, amount: BigUint) -> EsdtTokenPayment<Self::Api> {
        self.esdt_balance()
            .update(|previous_balance| *previous_balance -= &amount);
        EsdtTokenPayment::new(self.esdt_token_identifier().get(), 0u64, amount)
    }

    fn claim_sft(&self) -> ManagedVec<EsdtTokenPayment<Self::Api>> {
        let mut payments = ManagedVec::new();
        for key in self.sft_balance().keys() {
            payments.push(EsdtTokenPayment::new(
                self.sft_token_identifier().get(),
                key,
                BigUint::from(2u64),
            ));
            self.sft_balance()
                .entry(key)
                .and_modify(|previous_balance| *previous_balance -= BigUint::from(2u64));
        }
        payments
    }

    fn claim_nft(&self) -> EsdtTokenPayment<Self::Api> {
        let nonce = self.nft_balance().get_unchecked(0);
        self.nft_balance().swap_remove(0);

        EsdtTokenPayment::new(
            self.nft_token_identifier().get(),
            nonce,
            BigUint::from(1u64),
        )
    }

    #[view(get_esdt_token_identifier)]
    #[storage_mapper("esdt_token_identifier")]
    fn esdt_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(get_sft_token_identifier)]
    #[storage_mapper("sft_token_identifier")]
    fn sft_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(get_nft_token_identifier)]
    #[storage_mapper("nft_token_identifier")]
    fn nft_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(get_esdt_balance)]
    #[storage_mapper("esdt_balance")]
    fn esdt_balance(&self) -> SingleValueMapper<BigUint>; // total balance

    #[view(get_sft_balance)]
    #[storage_mapper("sft_balance")]
    fn sft_balance(&self) -> MapMapper<u64, BigUint>; // nonce -> balance

    #[view(get_nft_balance)]
    #[storage_mapper("nft_balance")]
    fn nft_balance(&self) -> VecMapper<u64>; // nonce only

    #[view(get_last_claim)]
    #[storage_mapper("last_claim2")]
    fn last_claim(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;
}
