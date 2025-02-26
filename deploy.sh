WALLET_PEM="wallet/deployer.pem"
PROXY="https://devnet-gateway.multiversx.com"
CHAIN_ID="D"

NFT_STAKING_WASM_PATH="nft-staking/output/nft-staking.wasm"
TRO_STAKING_WASM_PATH="tro-staking/output/tro-staking.wasm"

TRO_TOKEN_IDENTIFIER="TRO-9003a7"
LP_TOKEN_IDENTIFIERS=("TROLP1-2990e5" "TROLP2-291180")
NFT_COLLECTION_IDS=("XARTSFT-45f597" "XARTNFT-63b9ea")


DEMO_NFT_MINTER_WASM_PATH="demo-only-nft-minter/output/demo-only-nft-minter.wasm"
DEMO_NFT_MINTER_COLLECTION_NAME="xArtistsAIMegaWave"
DEMO_NFT_MINTER_COLLECTION_TICKER="XARTAIMW"

FAUCET_WASM_PATH="simple-devnet-faucet/output/simple-devnet-faucet.wasm"

deploy_nft_staking() {
    mxpy contract deploy --bytecode=$NFT_STAKING_WASM_PATH --recall-nonce --pem=${WALLET_PEM} --gas-limit=100000000 --send --outfile="deploy-nft-staking.interaction.json" --proxy=$PROXY --chain=$CHAIN_ID > /dev/null 2>&1
    jq -r '.contractAddress' deploy-nft-staking.interaction.json
}

upgrade_nft_staking() {
    NFT_STAKING_ADDRESS=$(jq -r '.contractAddress' deploy-nft-staking.interaction.json)
    mxpy contract upgrade ${NFT_STAKING_ADDRESS} --bytecode=$NFT_STAKING_WASM_PATH --recall-nonce --pem=${WALLET_PEM} --gas-limit=100000000 --send --outfile="upgrade-nft-staking.interaction.json" --proxy=$PROXY --chain=$CHAIN_ID > /dev/null 2>&1
    echo "NFT Staking contract upgraded successfully"
}

deploy_tro_staking() {
    mxpy contract deploy --bytecode=$TRO_STAKING_WASM_PATH --recall-nonce --pem=${WALLET_PEM} --gas-limit=100000000 --send --outfile="deploy-tro-staking.interaction.json" --proxy=$PROXY --chain=$CHAIN_ID --arguments str:$TRO_TOKEN_IDENTIFIER > /dev/null 2>&1
    jq -r '.contractAddress' deploy-tro-staking.interaction.json
}

upgrade_tro_staking() {
    TRO_STAKING_ADDRESS=$(jq -r '.contractAddress' deploy-tro-staking.interaction.json)
    mxpy contract upgrade ${TRO_STAKING_ADDRESS} --bytecode=$TRO_STAKING_WASM_PATH --recall-nonce --pem=${WALLET_PEM} --gas-limit=100000000 --send --outfile="upgrade-tro-staking.interaction.json" --proxy=$PROXY --chain=$CHAIN_ID > /dev/null 2>&1
    echo "TRO Staking contract upgraded successfully"
}

# TRO GOVERNANCE

add_tro_staking_whitelisted_lp_token() {
    TRO_STAKING_ADDRESS=$1
    LP_TOKEN_IDENTIFIER=$2
    mxpy contract call $TRO_STAKING_ADDRESS --function=addWhitelistedLpTokens --arguments str:$LP_TOKEN_IDENTIFIER --proxy=$PROXY --chain=$CHAIN_ID --pem=${WALLET_PEM} --recall-nonce --gas-limit=100000000 --send
}

setup_tro_staking_state() {
    TRO_STAKING_ADDRESS=$1
    for LP_TOKEN_IDENTIFIER in "${LP_TOKEN_IDENTIFIERS[@]}"
    do
        add_tro_staking_whitelisted_lp_token $TRO_STAKING_ADDRESS $LP_TOKEN_IDENTIFIER
        sleep 12
    done

    set_tro_token_identifier $TRO_STAKING_ADDRESS
}

setup_tro_governance_contract() {
    TRO_STAKING_ADDRESS=$(deploy_tro_staking)
    echo "TRO Staking deployed at: ${TRO_STAKING_ADDRESS}. Waiting 12 seconds..."
    sleep 12

    setup_tro_staking_state $TRO_STAKING_ADDRESS
    sleep 12
}

set_tro_token_identifier() {
    TRO_STAKING_ADDRESS=$1
    mxpy contract call $TRO_STAKING_ADDRESS --function=setTroTokenIdentifier --arguments str:$TRO_TOKEN_IDENTIFIER --proxy=$PROXY --chain=$CHAIN_ID --pem=${WALLET_PEM} --recall-nonce --gas-limit=100000000 --send
}

# NFT STAKING

add_nft_staking_whitelisted_nft_collection() {
    NFT_STAKING_ADDRESS=$1
    NFT_COLLECTION_ID=$2
    mxpy contract call $NFT_STAKING_ADDRESS --function=allowCollections --arguments str:$NFT_COLLECTION_ID --proxy=$PROXY --chain=$CHAIN_ID --pem=${WALLET_PEM} --recall-nonce --gas-limit=100000000 --send
}

setup_nft_staking_state() {
    NFT_STAKING_ADDRESS=$1
    for NFT_COLLECTION_ID in "${NFT_COLLECTION_IDS[@]}"
    do
        add_nft_staking_whitelisted_nft_collection $NFT_STAKING_ADDRESS $NFT_COLLECTION_ID
        sleep 12
    done
}

setup_nft_staking_contract() {
    NFT_STAKING_ADDRESS=$(deploy_nft_staking)
    echo "NFT Staking deployed at: ${NFT_STAKING_ADDRESS}. Waiting 12 seconds..."
    sleep 12

    setup_nft_staking_state $NFT_STAKING_ADDRESS
}


# DEMO NFT MINTER
deploy_demo_nft_minter() {
    mxpy contract deploy --bytecode=$DEMO_NFT_MINTER_WASM_PATH --recall-nonce --pem=${WALLET_PEM} --gas-limit=100000000 --send --outfile="deploy-demo-nft-minter.interaction.json" --proxy=$PROXY --chain=$CHAIN_ID > /dev/null 2>&1
    jq -r '.contractAddress' deploy-demo-nft-minter.interaction.json
}

issue_demo_nft_minter_collection() {
    DEMO_NFT_MINTER_ADDRESS=$1
    mxpy contract call $DEMO_NFT_MINTER_ADDRESS --function=issueToken --arguments str:$DEMO_NFT_MINTER_COLLECTION_NAME str:$DEMO_NFT_MINTER_COLLECTION_TICKER --proxy=$PROXY --chain=$CHAIN_ID --pem=${WALLET_PEM} --value=50000000000000000 --recall-nonce --gas-limit=80000000 --send
}

set_demo_nft_minter_local_roles() {
    DEMO_NFT_MINTER_ADDRESS=$1
    mxpy contract call $DEMO_NFT_MINTER_ADDRESS --function=setLocalRoles --proxy=$PROXY --chain=$CHAIN_ID --pem=${WALLET_PEM} --recall-nonce --gas-limit=100000000 --send
}

setup_demo_nft_minter_contract() {
    DEMO_NFT_MINTER_ADDRESS=$(deploy_demo_nft_minter)
    echo "Demo NFT Minter deployed at: ${DEMO_NFT_MINTER_ADDRESS}. Waiting 12 seconds..."
    sleep 12

    issue_demo_nft_minter_collection $DEMO_NFT_MINTER_ADDRESS
    sleep 60

    set_demo_nft_minter_local_roles $DEMO_NFT_MINTER_ADDRESS
    echo "Demo NFT Minter setup completed successfully"
}

# FAUCET

deploy_faucet() {
    mxpy contract deploy --bytecode=$FAUCET_WASM_PATH --recall-nonce --pem=${WALLET_PEM} --gas-limit=100000000 --send --outfile="deploy-faucet.interaction.json" --proxy=$PROXY --chain=$CHAIN_ID --arguments str:$TRO_TOKEN_IDENTIFIER str:${NFT_COLLECTION_IDS[0]} str:${NFT_COLLECTION_IDS[1]}
    echo "Faucet deployed at: $(jq -r '.contractAddress' deploy-faucet.interaction.json)"
}

upgrade_faucet() {
    FAUCET_ADDRESS=$1
    mxpy contract upgrade $FAUCET_ADDRESS --bytecode=$FAUCET_WASM_PATH --recall-nonce --pem=${WALLET_PEM} --gas-limit=100000000 --send --outfile="upgrade-faucet.interaction.json" --proxy=$PROXY --chain=$CHAIN_ID
}

deposit_tro_tokens_to_faucet() {
    FAUCET_ADDRESS=$1
    mxpy contract call $FAUCET_ADDRESS --function=deposit --proxy=$PROXY --chain=$CHAIN_ID --pem=${WALLET_PEM} --recall-nonce --gas-limit=100000000 --send --token-transfers $TRO_TOKEN_IDENTIFIER 1000000000000000000000000
}

claim_from_faucet() {
    FAUCET_ADDRESS=$1
    mxpy contract call $FAUCET_ADDRESS --function=claim --proxy=$PROXY --chain=$CHAIN_ID --pem=${WALLET_PEM} --recall-nonce --gas-limit=100000000 --send
}

# MAIN

# setup_tro_governance_contract
# setup_nft_staking_contract

# upgrade_nft_staking
# upgrade_tro_staking


# setup_demo_nft_minter_contract
# address=$(deploy_faucet)
# sleep 12
# deposit_tro_tokens_to_faucet erd1qqqqqqqqqqqqqpgqjzn7zjyevwez8n0zfevpvnrwyp2ln879yj7sj8354t
upgrade_faucet erd1qqqqqqqqqqqqqpgqjzn7zjyevwez8n0zfevpvnrwyp2ln879yj7sj8354t
sleep 12
claim_from_faucet erd1qqqqqqqqqqqqqpgqjzn7zjyevwez8n0zfevpvnrwyp2ln879yj7sj8354t
