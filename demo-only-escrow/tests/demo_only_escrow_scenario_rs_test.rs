use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/demo-only-escrow.mxsc.json", demo_only_escrow::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/demo_only_escrow.scen.json");
}
