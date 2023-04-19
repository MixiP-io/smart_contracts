#![no_std]

mod error;
mod payment_contract_info;
mod storage_types;

use error::ContractError;
use payment_contract_info::{has_contact_info, PaymentContractInfo};
use soroban_sdk::{contractimpl, panic_with_error, Address, Env};

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
}

mod test;
