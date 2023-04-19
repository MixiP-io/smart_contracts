#![no_std]

mod asset;
mod error;
mod metadata;
mod payment_contract_info;
mod storage_types;

use error::ContractError;
use metadata::{is_contract_active, is_contract_with_state};
use payment_contract_info::{has_contact_info, PaymentContractInfo};
use soroban_sdk::{contractimpl, panic_with_error, Address, Bytes, Env, Map};

pub struct PaymentContract;

#[contractimpl]
impl PaymentContract {
    pub fn initialize(env: Env, contract_info: PaymentContractInfo, creator: Address) {
        if has_contact_info(&env) {
            panic_with_error!(env, ContractError::AlreadyInitialized);
        }
        payment_contract_info::write_contract_info(&env, &contract_info);
        payment_contract_info::write_creator(&env, &creator)
    }

    pub fn update_creator(env: Env, creator: Address) {
        payment_contract_info::get_contract_manager_address(&env).require_auth();
        payment_contract_info::write_creator(&env, &creator)
    }

    pub fn sign_contract(env: Env, date: u64) {
        if is_contract_with_state(&env) {
            panic_with_error!(env, ContractError::AlreadyInProgress)
        }
        let creator = payment_contract_info::get_creator(&env);
        creator.require_auth();
        metadata::sign_contract(&env, &date);
    }

    pub fn submit_asset(env: Env, assets: Map<Bytes, Bytes>, submission_date: u64) {
        if !is_contract_active(&env) {
            panic_with_error!(env, ContractError::ContractNotActive)
        }
        let creator = payment_contract_info::get_creator(&env);
        creator.require_auth();
        asset::store_assets(&env, assets, submission_date)
    }
}

mod test;
