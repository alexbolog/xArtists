use test_setup::setup_world_with_contract;

// pub mod permissions;
// pub mod stake;
pub mod helpers;
pub mod scenarios;
pub mod test_setup;
// pub mod unstake;
// pub mod rewards;

#[test]
fn deploy_should_succeed() {
    setup_world_with_contract();
}
