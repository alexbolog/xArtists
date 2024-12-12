multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode)]
pub struct StakeEvent<M: ManagedTypeApi> {
    caller: ManagedAddress<M>,
    payments: MultiEsdtPayment<M>,
}

#[allow(type_alias_bounds)]
pub type UnstakeEvent<M: ManagedTypeApi> = StakeEvent<M>;

#[multiversx_sc::module]
pub trait EventsModule: crate::storage::StorageModule {
    fn emit_stake_event(&self, caller: &ManagedAddress, payments: &MultiEsdtPayment<Self::Api>) {
        let epoch = self.blockchain().get_block_epoch();
        let stake_event = StakeEvent {
            caller: caller.clone(),
            payments: payments.clone(),
        };

        self.stake_event(
            caller,
            epoch,
            self.blockchain().get_block_nonce(),
            self.blockchain().get_block_timestamp(),
            &stake_event,
        );
    }

    #[event("stake")]
    fn stake_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] block: u64,
        #[indexed] timestamp: u64,
        stake_event: &StakeEvent<Self::Api>,
    );

    fn emit_unstake_event(&self, caller: &ManagedAddress, payments: &MultiEsdtPayment<Self::Api>) {
        let epoch = self.blockchain().get_block_epoch();
        let unstake_event = UnstakeEvent {
            caller: caller.clone(),
            payments: payments.clone(),
        };

        self.unstake_event(
            caller,
            epoch,
            self.blockchain().get_block_nonce(),
            self.blockchain().get_block_timestamp(),
            &unstake_event,
        );
    }

    #[event("unstake")]
    fn unstake_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] block: u64,
        #[indexed] timestamp: u64,
        unstake_event: &UnstakeEvent<Self::Api>,
    );
}
