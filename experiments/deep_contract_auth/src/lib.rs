#![no_std]
/// This example demonstrates how a contract can authorize deep subcontract
/// calls on its behalf.
///
/// By default, only direct calls that contract makes are authorized. However,
/// in some scenarios one may want to authorize a deeper call (a common example
/// would be token transfer).
///
/// Here we provide the abstract example: contract A calls contract B, then
/// contract B calls contract C. Both contract B and contract C `require_auth`
/// for contract A address and contract A provides proper authorization to make
/// the calls succeed.

mod contract_a {

    use soroban_sdk::{
        auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
        contract, contractimpl, vec, Address, Env, IntoVal, Symbol, log,
    };

    use crate::contract_b::ContractBClient;

    #[contract]
    pub struct ContractA;

    #[contractimpl]
    impl ContractA {
        pub fn call_b(env: Env, contract_b_address: Address, contract_c_address: Address) {
            // This function authorizes sub-contract calls that are made from
            // the next call A performs on behalf of the current contract.
            // Note, that these *do not* contain direct calls because they are
            // always authorized. So here we pre-authorize call of contract C
            // that will be performed by contract B.
            env.authorize_as_current_contract(vec![
                &env,
                InvokerContractAuthEntry::Contract(SubContractInvocation {
                    context: ContractContext {
                        contract: contract_c_address.clone(),
                        fn_name: Symbol::new(&env, "authorized_fn_c"),
                        args: (env.current_contract_address(),).into_val(&env),
                    },
                    // `sub_invocations` can be used to authorize even deeper
                    // calls.
                    sub_invocations: vec![&env],
                }),
            ]);
            let client = ContractBClient::new(&env, &contract_b_address);
            client.authorized_fn_b(&env.current_contract_address(), &contract_c_address);
        }

        pub fn call_d(env: Env, contract_b_address: Address, contract_c_address: Address, contract_d_address: Address) {
            let call_c_on_behalf_of_a = ContractContext {
                contract: contract_c_address.clone(),
                fn_name: Symbol::new(&env, "authorized_fn_c"),
                args: (env.current_contract_address(),).into_val(&env),
            };

            let allow_direct_call_to_c = InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: call_c_on_behalf_of_a.clone(),
                // `sub_invocations` can be used to authorize even deeper
                // calls.
                sub_invocations: vec![&env], });

            env.authorize_as_current_contract(vec![
                &env,
                allow_direct_call_to_c,
            ]);
            log!(&env, "ContractA calling B::call_d()");
            let client = ContractBClient::new(&env, &contract_b_address);
            client.call_d(&env.current_contract_address(), &contract_c_address, &contract_d_address);
        }

        pub fn call_d_twice_single_auth(env: Env, contract_b_address: Address, contract_c_address: Address, contract_d_address: Address) {
            let call_c_on_behalf_of_a = ContractContext {
                contract: contract_c_address.clone(),
                fn_name: Symbol::new(&env, "authorized_fn_c"),
                args: (env.current_contract_address(),).into_val(&env),
            };

            let allow_direct_call_to_c = InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: call_c_on_behalf_of_a.clone(),
                sub_invocations: vec![&env], });

            env.authorize_as_current_contract(vec![
                &env,
                allow_direct_call_to_c,
            ]);
            let client = ContractBClient::new(&env, &contract_b_address);
            log!(&env, "ContractA calling B::call_d() first time");
            client.call_d(&env.current_contract_address(), &contract_c_address, &contract_d_address);
        
            log!(&env, "ContractA calling B::call_d() second time");
            client.call_d(&env.current_contract_address(), &contract_c_address, &contract_d_address);
        }

        pub fn call_d_twice_double_auth(env: Env, contract_b_address: Address, contract_c_address: Address, contract_d_address: Address) {
            let call_c_on_behalf_of_a = ContractContext {
                contract: contract_c_address.clone(),
                fn_name: Symbol::new(&env, "authorized_fn_c"),
                args: (env.current_contract_address(),).into_val(&env),
            };

            let allow_direct_call_to_c = InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: call_c_on_behalf_of_a.clone(),
                sub_invocations: vec![&env], });

            env.authorize_as_current_contract(vec![
                &env,
                allow_direct_call_to_c.clone(),
            ]);
            let client = ContractBClient::new(&env, &contract_b_address);
            log!(&env, "ContractA calling B::call_d() first time");
            client.call_d(&env.current_contract_address(), &contract_c_address, &contract_d_address);
            env.authorize_as_current_contract(vec![
                &env,
                allow_direct_call_to_c,
            ]);
            log!(&env, "ContractA calling B::call_d() second time");
            client.call_d(&env.current_contract_address(), &contract_c_address, &contract_d_address);
        }
    }
}

mod contract_b {
    use soroban_sdk::{contract, contractimpl, Address, Env, log};

    use crate::{contract_c::ContractCClient, contract_d::ContractDClient};

    #[contract]
    pub struct ContractB;

    #[contractimpl]
    impl ContractB {
        pub fn authorized_fn_b(env: Env, authorizer: Address, contract_c_address: Address) {
            authorizer.require_auth();
            let client = ContractCClient::new(&env, &contract_c_address);
            client.authorized_fn_c(&authorizer);
        }

        pub fn call_d(env: Env, authorizer: Address, contract_c_address: Address, contract_d_address: Address) {
            log!(&env, "ContractB calling D::call_c()");
            authorizer.require_auth();
            let client = ContractDClient::new(&env, &contract_d_address);
            client.call_c(&authorizer, &env.current_contract_address(), &contract_c_address);
        }
    }
}
mod contract_c {

    use soroban_sdk::{contract, contractimpl, Address, Env, log};

    #[contract]
    pub struct ContractC;

    #[contractimpl]
    impl ContractC {
        pub fn authorized_fn_c(env: Env, authorizer: Address) {
            log!(&env, "ContractC::authorized_fn_c()");
            authorizer.require_auth();
        }
    }
}

mod contract_d {

    use soroban_sdk::{contract, contractimpl, Address, Env, log};

    use crate::contract_c::ContractCClient;

    #[contract]
    pub struct ContractD;

    #[contractimpl]
    impl ContractD {
        pub fn call_c(env: Env, authorizer: Address, contract_b_address: Address, contract_c_address: Address) {
            contract_b_address.require_auth(); // this should always succeed when called by B
            log!(&env, "ContractD calling C::authorized_fn_c()");
            let client = ContractCClient::new(&env, &contract_c_address);
            client.authorized_fn_c(&authorizer);
        }
    }
}

mod test;
