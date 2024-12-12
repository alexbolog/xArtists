#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

mod stake;
mod storage;

/// $TRO staking smart contract
/// Users can stake $TRO and LP tokens in order to:
/// - participate in the xArtist governance mechanism
/// - earn rewards? TODO: check with team
#[multiversx_sc::contract]
pub trait TroStaking: storage::StorageModule + stake::StakeModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
