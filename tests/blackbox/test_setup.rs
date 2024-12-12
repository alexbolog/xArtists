use multiversx_sc_scenario::imports::*;

use crate::config::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(CODE_PATH, tro_staking::ContractBuilder);
    blockchain
}

fn add_account(world: &mut ScenarioWorld, address: TestAddress) {
    world
        .account(address)
        .nonce(1)
        .balance(INITIAL_TOKEN_BALANCE)
        .esdt_balance(TRO_TOKEN_ID, INITIAL_TOKEN_BALANCE)
        .esdt_balance(LP_TOKEN_ID_1, INITIAL_TOKEN_BALANCE)
        .esdt_balance(LP_TOKEN_ID_2, INITIAL_TOKEN_BALANCE)
        .esdt_balance(LP_TOKEN_ID_3, INITIAL_TOKEN_BALANCE)
        .esdt_balance(UNSUPPORTED_LP_TOKEN_ID, INITIAL_TOKEN_BALANCE);
}

fn add_supported_lp_token(world: &mut ScenarioWorld, token_ids: &[&TestTokenIdentifier]) {
    let mut args = MultiValueEncoded::new();
    for token_id in token_ids {
        args.push(token_id.to_token_identifier());
    }

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .add_whitelisted_lp_tokens(args)
        .returns(ExpectStatus(0u64))
        .run();
}

fn deploy_contract(world: &mut ScenarioWorld) {
    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .init(TRO_TOKEN_ID, MultiValueEncoded::new())
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, SC_ADDRESS);
}

fn apply_blackbox_setup(world: &mut ScenarioWorld) {
    add_account(world, OWNER_ADDRESS);
    add_account(world, USER_ADDRESS);
}

pub fn setup_world() -> ScenarioWorld {
    let mut world = world();
    apply_blackbox_setup(&mut world);

    world
}

pub fn setup_world_with_contract() -> ScenarioWorld {
    let mut world = setup_world();
    deploy_contract(&mut world);
    add_supported_lp_token(
        &mut world,
        &[&LP_TOKEN_ID_1, &LP_TOKEN_ID_2, &LP_TOKEN_ID_3],
    );
    world
}
