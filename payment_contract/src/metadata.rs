//! Module MetaData
//!
//! Module for obtaining and modifying the metadata of the contract.
use crate::storage_types::{ContractState, DataKey};
use soroban_sdk::Env;

pub fn sign_contract(env: &Env, date: &u64) {
    let state_key = DataKey::ContractState;
    let acceptance_date_key = DataKey::DateOfAcceptance;
    env.storage().set(&state_key, &ContractState::Active);
    env.storage().set(&acceptance_date_key, date);
}

pub fn is_contract_with_state(env: &Env) -> bool {
    let state_key = DataKey::ContractState;
    env.storage().has(&state_key)
}

pub fn is_contract_active(env: &Env) -> bool {
    let state_key = DataKey::ContractState;
    match env.storage().get(&state_key) {
        Some(state) => matches!(state.unwrap(), ContractState::Active),
        None => false,
    }
}

pub(crate) fn update_fee(env: &Env, amount: &i128) {
    let fee_key = DataKey::TotalFeeAmount;
    let fee: i128 = match env.storage().get(&fee_key) {
        Some(fee) => fee.unwrap(),
        None => 0,
    };
    env.storage().set(&fee_key, &(fee + amount))
}
