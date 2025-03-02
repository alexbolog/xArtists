use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/demo-only-nft-minter.mxsc.json", demo_only_nft_minter::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/demo_only_nft_minter.scen.json");
}
