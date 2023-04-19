#![cfg(test)]

use crate::{
    payment_contract_info::{ContractManager, ContractType, PaymentContractInfo, PaymentMethod},
    PaymentContract, PaymentContractClient,
};
use soroban_sdk::{testutils::Address as _, Address, Bytes, Env, IntoVal};

fn create_payment_contract(
    e: &Env,
    payment_contract_info: &PaymentContractInfo,
) -> PaymentContractClient {
    let payment_contract =
        PaymentContractClient::new(e, &e.register_contract(None, PaymentContract {}));
    payment_contract.initialize(payment_contract_info);
    payment_contract
}

struct PaymentContractTest {
    env: Env,
    payment_contract_info: PaymentContractInfo,
}

impl PaymentContractTest {
    fn setup() -> Self {
        let env: Env = Default::default();
        let contract_manager_address = Address::random(&env);
        let company_id: Bytes = "ID-001".into_val(&env);
        let project_id: Bytes = "ID-001".into_val(&env);
        let contract_name: Bytes = "Test Contract Name".into_val(&env);
        let contract_manager: ContractManager = ContractManager {
            address: contract_manager_address,
            name: "John Doe".into_val(&env),
            job_position: "Product owner".into_val(&env),
            physical_address: "Some address".into_val(&env),
        };
        let payment_contract_info = PaymentContractInfo {
            contract_manager,
            company_id,
            project_id,
            contract_name,
            payment_method: PaymentMethod::Native,
            asset_payment_amount: 5,
            creation_date: 1681917160,
            deadline: 1684546903,
            payment_time: 0,
            contract_type: ContractType::Milestones,
            start_date: 1682003560,
            scope_of_work: "scope_of_work text".into_val(&env),
            rights_royalties: "rights_royalties text".into_val(&env),
        };

        PaymentContractTest {
            env,
            payment_contract_info,
        }
    }
}

#[test]
fn test_successful_contract_initialize() {
    let test = PaymentContractTest::setup();

    create_payment_contract(&test.env, &test.payment_contract_info);
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_initialize_an_already_initialized_payment_contract() {
    let test = PaymentContractTest::setup();
    let payment_contract = create_payment_contract(&test.env, &test.payment_contract_info);
    payment_contract.initialize(&test.payment_contract_info);
}
