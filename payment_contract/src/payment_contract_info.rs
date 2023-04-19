//! Module PaymentContractInfo
//!
//! Module responsible of managing `PaymentContractInfo` and defining its corresponding struct.
use soroban_sdk::{contracttype, Address, Bytes};

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
///Struct that stores the necessary information for the contract
pub struct PaymentContractInfo {
    /// The contract manager of the contract
    pub contract_manager: ContractManager,
    /// The identification of the company in an off chain storage
    pub company_id: Bytes,
    /// The identification of the project in an off chain storage
    pub project_id: Bytes,
    /// The identification of the contract in an off chain storage
    pub contract_name: Bytes,
    /// The way the payment will be executed (only native (xlm) for now)
    pub payment_method: PaymentMethod,
    /// The payment amount for each approved asset
    pub asset_payment_amount: u64,
    /// Contract creation date
    pub creation_date: u64,
    /// The accorded date to start the contract execution
    pub start_date: u64,
    /// The last day on which assets could be uploaded
    pub deadline: u64,
    /// Conditions of the contract
    pub scope_of_work: Bytes,
    pub rights_royalties: Bytes,
    pub payment_time: u64,
    pub contract_type: ContractType,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PaymentMethod {
    /// XLM
    Native,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ContractType {
    FixedPrice,
    Milestones,
    Licensing,
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
pub struct ContractManager {
    address: Address,
    name: Bytes,
    title: Bytes,
    physical_address: Bytes,
}
