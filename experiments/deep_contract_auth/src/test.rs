#![cfg(test)]

use std::println;

use soroban_sdk::Env;

use crate::{
    contract_a::{ContractA, ContractAClient},
    contract_b::ContractB,
    contract_c::ContractC,
    contract_d::ContractD,
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
    client.call_d_through_b(&b_address, &c_address, &d_address);
}

/// A calls B calls D calls C, with C require_auth'ing from A
/// A only gives B authorization, and has no knowledge of D
/// In this case A calls B twice, whilst only passing one authorization for C.
#[test]
#[should_panic]
fn test_indirect_double_call_single_auth() {
    let env = Env::default();
    let a_address = env.register_contract(None, ContractA);
    let b_address = env.register_contract(None, ContractB);
    let c_address = env.register_contract(None, ContractC);
    let d_address = env.register_contract(None, ContractD);
    let client = ContractAClient::new(&env, &a_address);
    client.call_b_twice_single_auth(&b_address, &c_address, &d_address);
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
    client.call_b_twice_double_auth(&b_address, &c_address, &d_address);
}

/// A authorizes B to call C twice, and A calls B once.
/// B calls C twice, and C requires authorization from A.
/// The authorization is passed as two Root |InvokerContractAuthEntry|'s
/// in |authorize_as_current_contract|.
#[test]
fn test_double_authorization() {
    let env = Env::default();
    let a_address = env.register_contract(None, ContractA);
    let b_address = env.register_contract(None, ContractB);
    let c_address = env.register_contract(None, ContractC);
    let d_address = env.register_contract(None, ContractD);
    let client = ContractAClient::new(&env, &a_address);
    client.auth_double_call(&b_address, &c_address, &d_address);

    println!("{:?}", env.auths());
}

/// A gives B an authorization of type [Nonsense -> C]; because the Nonsense
// does not match, C does not match either, so this fails.
#[test]
#[should_panic]
fn test_sub_invocation_bad_root() {
    let env = Env::default();
    let a_address = env.register_contract(None, ContractA);
    let b_address = env.register_contract(None, ContractB);
    let c_address = env.register_contract(None, ContractC);
    let d_address = env.register_contract(None, ContractD);
    let client = ContractAClient::new(&env, &a_address);
    client.auth_sub_invocation_bad_root(&b_address, &c_address, &d_address);

    println!("{:?}", env.auths());
}

// A authorizes D (root invocation) and authorizes D to call C (sub-invocation).
// The call stack is A -> B -> D -> C.
#[test]
fn test_sub_invocation_good_root() {
    let env = Env::default();
    let a_address = env.register_contract(None, ContractA);
    let b_address = env.register_contract(None, ContractB);
    let c_address = env.register_contract(None, ContractC);
    let d_address = env.register_contract(None, ContractD);
    let client = ContractAClient::new(&env, &a_address);
    client.auth_sub_invocation(&b_address, &c_address, &d_address);

    println!("{:?}", env.auths());
}
