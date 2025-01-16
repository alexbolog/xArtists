use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/nft-staking.mxsc.json", nft_staking::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/nft_staking.scen.json");
}
