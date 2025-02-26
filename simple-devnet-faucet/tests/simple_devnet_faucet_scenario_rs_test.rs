use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/simple-devnet-faucet.mxsc.json", simple_devnet_faucet::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/simple_devnet_faucet.scen.json");
}
