use crate::config::*;
use multiversx_sc_scenario::imports::*;

use super::helpers::send_allow_collection_tx;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(CODE_PATH, nft_staking::ContractBuilder);
    blockchain
}

pub fn setup_world_with_contract() -> ScenarioWorld {
    let mut world = setup_world();
    deploy_contract(&mut world);
    send_allow_collection_tx(&mut world, &NFT_TOKEN_ID);
    send_allow_collection_tx(&mut world, &SFT_TOKEN_ID);

    world
}

fn deploy_contract(world: &mut ScenarioWorld) {
    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(nft_staking::proxy::NftStakingProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS);
}


pub fn setup_world() -> ScenarioWorld {
    let mut world = world();
    apply_blackbox_setup(&mut world);

    world
}

fn apply_blackbox_setup(world: &mut ScenarioWorld) {
    add_accounts(world);
}

fn add_accounts(world: &mut ScenarioWorld) {
    // Owner account
    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .balance(INITIAL_ESDT_BALANCE)
        .esdt_balance(REWARD_TOKEN_ID_1, INITIAL_ESDT_BALANCE)
        .esdt_balance(REWARD_TOKEN_ID_2, INITIAL_ESDT_BALANCE)
        .esdt_nft_balance(SFT_TOKEN_ID, NFTSFT_NONCES[0], INITIAL_SFT_BALANCE, managed_buffer!(b""))
        .esdt_nft_balance(SFT_TOKEN_ID, NFTSFT_NONCES[1], INITIAL_SFT_BALANCE, managed_buffer!(b""))
        .esdt_nft_balance(SFT_TOKEN_ID, NFTSFT_NONCES[2], INITIAL_SFT_BALANCE, managed_buffer!(b""))
        .esdt_nft_balance(SFT_TOKEN_ID, NFTSFT_NONCES[3], INITIAL_SFT_BALANCE, managed_buffer!(b""))
        .esdt_nft_balance(SFT_TOKEN_ID, NFTSFT_NONCES[4], INITIAL_SFT_BALANCE, managed_buffer!(b""));

    // User account
    world
        .account(USER_ADDRESS)
        .nonce(1)
        .balance(INITIAL_ESDT_BALANCE)
        .esdt_nft_balance(NFT_TOKEN_ID, NFTSFT_NONCES[0], 1, managed_buffer!(b""))
        .esdt_nft_balance(NFT_TOKEN_ID, NFTSFT_NONCES[1], 1, managed_buffer!(b""))
        .esdt_nft_balance(NFT_TOKEN_ID, NFTSFT_NONCES[2], 1, managed_buffer!(b""))
        .esdt_nft_balance(NFT_TOKEN_ID, NFTSFT_NONCES[3], 1, managed_buffer!(b""))
        .esdt_nft_balance(NFT_TOKEN_ID, NFTSFT_NONCES[4], 1, managed_buffer!(b""))
        .esdt_nft_balance(SFT_TOKEN_ID, NFTSFT_NONCES[0], INITIAL_SFT_BALANCE, managed_buffer!(b""))
        .esdt_nft_balance(SFT_TOKEN_ID, NFTSFT_NONCES[1], INITIAL_SFT_BALANCE, managed_buffer!(b""))
        .esdt_nft_balance(SFT_TOKEN_ID, NFTSFT_NONCES[2], INITIAL_SFT_BALANCE, managed_buffer!(b""))
        .esdt_nft_balance(SFT_TOKEN_ID, NFTSFT_NONCES[3], INITIAL_SFT_BALANCE, managed_buffer!(b""))
        .esdt_nft_balance(SFT_TOKEN_ID, NFTSFT_NONCES[4], INITIAL_SFT_BALANCE, managed_buffer!(b""))
        .esdt_nft_balance(UNSUPPORTED_NFT_TOKEN_ID, NFTSFT_NONCES[0], 1, managed_buffer!(b""));
}



