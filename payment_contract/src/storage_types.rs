//! Module StorageTypes
//!
//! Module that defines the set of keys that can be used to access and store data within the contract.
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// Stores all the metadata of the contract as a `PaymentContractInfo` struct
    PaymentContractInfo,
}
