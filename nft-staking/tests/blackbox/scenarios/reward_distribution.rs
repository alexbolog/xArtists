use nft_staking::reward::reward_rate::REWARD_RATE_DENOMINATION;

use crate::blackbox::{
    helpers::check_distribution_amount_per_round, test_setup::setup_world_with_contract,
};

#[test]
fn planned_reward_distribution_round_distribution_amount_calculation_should_work() {
    let mut world = setup_world_with_contract();

    check_distribution_amount_per_round(
        &mut world,
        0,
        10,
        REWARD_RATE_DENOMINATION,
        REWARD_RATE_DENOMINATION / 10,
    );
}
