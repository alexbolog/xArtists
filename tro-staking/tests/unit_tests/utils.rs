use crate::config::*;
use multiversx_sc_scenario::api::SingleTxApi;
use tro_staking::TroStaking;

pub fn init_staking_contract() -> tro_staking::ContractObj<SingleTxApi> {
    let staking_contract = tro_staking::contract_obj::<SingleTxApi>();
    staking_contract.init(TRO_TOKEN_ID.to_token_identifier());

    staking_contract
}
