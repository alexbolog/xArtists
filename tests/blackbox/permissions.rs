use multiversx_sc_scenario::imports::*;

use crate::config::*;

use super::test_setup::setup_world_with_contract;

#[test]
fn owner_calling_add_whitelisted_lp_tokens_should_succeed() {
    let mut world = setup_world_with_contract();

    let mut args = MultiValueEncoded::new();
    args.push(UNSUPPORTED_LP_TOKEN_ID.to_token_identifier());

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .add_whitelisted_lp_tokens(args)
        .returns(ExpectStatus(0u64))
        .run();
}

#[test]
fn user_calling_add_whitelisted_lp_tokens_should_fail() {
    let mut world = setup_world_with_contract();

    let mut args = MultiValueEncoded::new();
    args.push(UNSUPPORTED_LP_TOKEN_ID.to_token_identifier());

    world
        .tx()
        .from(USER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(tro_staking::proxy::TroStakingProxy)
        .add_whitelisted_lp_tokens(args)
        .returns(ExpectStatus(4u64))
        .run();
}
