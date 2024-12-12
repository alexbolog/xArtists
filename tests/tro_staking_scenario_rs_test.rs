use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/tro-staking.mxsc.json", tro_staking::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/tro_staking.scen.json");
}
