use test_setup::setup_world_with_contract;

pub mod permissions;
pub mod stake;
pub mod test_setup;

#[test]
fn deploy_should_succeed() {
    setup_world_with_contract();
}
