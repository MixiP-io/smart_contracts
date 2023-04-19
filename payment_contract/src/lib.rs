#![no_std]

mod payment_contract_info;

use soroban_sdk::contractimpl;

pub struct PaymentContract;

#[contractimpl]
impl PaymentContract {}
