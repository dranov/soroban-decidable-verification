#![cfg(test)]

use soroban_sdk::Env;

use crate::{
    contract_a::{ContractA, ContractAClient},
    contract_b::ContractB,
    contract_c::ContractC, contract_d::ContractD,
};
extern crate std;

/// A calls B calls C, with C require_auth'ing from A
#[test]
fn test_direct_chain_call() {
    let env = Env::default();
    let a_address = env.register_contract(None, ContractA);
    let b_address = env.register_contract(None, ContractB);
    let c_address = env.register_contract(None, ContractC);
    let client = ContractAClient::new(&env, &a_address);
    client.call_b(&b_address, &c_address);
}

/// A calls B calls D calls C, with C require_auth'ing from A
/// A only gives B authorization, and has no knowledge of D
#[test]
fn test_indirect_call() {
    let env = Env::default();
    let a_address = env.register_contract(None, ContractA);
    let b_address = env.register_contract(None, ContractB);
    let c_address = env.register_contract(None, ContractC);
    let d_address = env.register_contract(None, ContractD);
    let client = ContractAClient::new(&env, &a_address);
    client.call_d(&b_address, &c_address, &d_address);
}

/// A calls B calls D calls C, with C require_auth'ing from A
/// A only gives B authorization, and has no knowledge of D
/// In this case A call's B twice, whilst only passing one authorization for C.
#[test]
#[should_panic]
fn test_indirect_double_call_single_auth() {
    let env = Env::default();
    let a_address = env.register_contract(None, ContractA);
    let b_address = env.register_contract(None, ContractB);
    let c_address = env.register_contract(None, ContractC);
    let d_address = env.register_contract(None, ContractD);
    let client = ContractAClient::new(&env, &a_address);
    client.call_d_twice_single_auth(&b_address, &c_address, &d_address);
}

/// A calls B calls D calls C, with C require_auth'ing from A
/// A only gives B authorization, and has no knowledge of D
/// In this case A authorizes C twice.
#[test]
fn test_indirect_double_call() {
    let env = Env::default();
    let a_address = env.register_contract(None, ContractA);
    let b_address = env.register_contract(None, ContractB);
    let c_address = env.register_contract(None, ContractC);
    let d_address = env.register_contract(None, ContractD);
    let client = ContractAClient::new(&env, &a_address);
    client.call_d_twice_double_auth(&b_address, &c_address, &d_address);
}

