WALLET_PEM="wallet/deployer.pem"
PROXY="https://devnet-gateway.multiversx.com"
CHAIN_ID="D"

NFT_STAKING_WASM_PATH="nft-staking/output/nft-staking.wasm"
TRO_STAKING_WASM_PATH="tro-staking/output/tro-staking.wasm"

TRO_TOKEN_IDENTIFIER="TRO-9003a7"
LP_TOKEN_IDENTIFIERS=("TROLP1-2990e5" "TROLP2-291180")
NFT_COLLECTION_IDS=("XARTSFT-45f597" "XARTNFT-63b9ea")

read_nft_staking_address() {
    jq -r '.contractAddress' deploy-nft-staking.interaction.json
}

read_tro_staking_address() {
    jq -r '.contractAddress' deploy-tro-staking.interaction.json
}

# NFT STAKING
distribute_nft_staking_rewards() {
    NFT_STAKING_ADDRESS=$1
    mxpy contract call $NFT_STAKING_ADDRESS --function=distributeRewards --token-transfers $TRO_TOKEN_IDENTIFIER 1000000000000000000000 --proxy=$PROXY --chain=$CHAIN_ID --pem=${WALLET_PEM} --recall-nonce --gas-limit=100000000 --send
}

# MAIN
distribute_nft_staking_rewards $(read_nft_staking_address)
