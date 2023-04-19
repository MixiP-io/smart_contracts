//! Module Error
//!
//! Module that groups the errors within the contract and assigns them a code
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// Error that indicates the contract was already initialized
    AlreadyInitialized = 1,
    /// Error that indicates the contract is already accepted, finished or unaccepted for the creator
    AlreadyInProgress = 2,
}
