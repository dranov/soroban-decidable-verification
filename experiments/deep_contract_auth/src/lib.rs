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
        contract, contractimpl, log, vec, Address, Env, IntoVal, Symbol,
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

        pub fn call_d_through_b(
            env: Env,
            contract_b_address: Address,
            contract_c_address: Address,
            contract_d_address: Address,
        ) {
            let call_c_on_behalf_of_a = ContractContext {
                contract: contract_c_address.clone(),
                fn_name: Symbol::new(&env, "authorized_fn_c"),
                args: (env.current_contract_address(),).into_val(&env),
            };

            let allow_call_to_c = InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: call_c_on_behalf_of_a.clone(),
                // `sub_invocations` can be used to authorize even deeper
                // calls.
                sub_invocations: vec![&env],
            });

            env.authorize_as_current_contract(vec![&env, allow_call_to_c]);
            log!(&env, "ContractA calling B::call_d()");
            let client = ContractBClient::new(&env, &contract_b_address);
            client.call_d(
                &env.current_contract_address(),
                &contract_c_address,
                &contract_d_address,
            );
        }

        pub fn call_b_twice_single_auth(
            env: Env,
            contract_b_address: Address,
            contract_c_address: Address,
            contract_d_address: Address,
        ) {
            let call_c_on_behalf_of_a = ContractContext {
                contract: contract_c_address.clone(),
                fn_name: Symbol::new(&env, "authorized_fn_c"),
                args: (env.current_contract_address(),).into_val(&env),
            };

            let allow_call_to_c = InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: call_c_on_behalf_of_a.clone(),
                sub_invocations: vec![&env],
            });

            env.authorize_as_current_contract(vec![&env, allow_call_to_c]);
            let client = ContractBClient::new(&env, &contract_b_address);
            log!(&env, "ContractA calling B::call_d() first time");
            client.call_d(
                &env.current_contract_address(),
                &contract_c_address,
                &contract_d_address,
            );

            log!(&env, "ContractA calling B::call_d() second time");
            client.call_d(
                &env.current_contract_address(),
                &contract_c_address,
                &contract_d_address,
            );
        }

        pub fn call_b_twice_double_auth(
            env: Env,
            contract_b_address: Address,
            contract_c_address: Address,
            contract_d_address: Address,
        ) {
            let call_c_on_behalf_of_a = ContractContext {
                contract: contract_c_address.clone(),
                fn_name: Symbol::new(&env, "authorized_fn_c"),
                args: (env.current_contract_address(),).into_val(&env),
            };

            let allow_call_to_c = InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: call_c_on_behalf_of_a.clone(),
                sub_invocations: vec![&env],
            });

            env.authorize_as_current_contract(vec![&env, allow_call_to_c.clone()]);
            let client = ContractBClient::new(&env, &contract_b_address);
            log!(&env, "ContractA calling B::call_d() first time");
            client.call_d(
                &env.current_contract_address(),
                &contract_c_address,
                &contract_d_address,
            );
            env.authorize_as_current_contract(vec![&env, allow_call_to_c]);
            log!(&env, "ContractA calling B::call_d() second time");
            client.call_d(
                &env.current_contract_address(),
                &contract_c_address,
                &contract_d_address,
            );
        }

        pub fn auth_double_call(
            env: Env,
            contract_b_address: Address,
            contract_c_address: Address,
            contract_d_address: Address,
        ) {
            let call_c_on_behalf_of_a = ContractContext {
                contract: contract_c_address.clone(),
                fn_name: Symbol::new(&env, "authorized_fn_c"),
                args: (env.current_contract_address(),).into_val(&env),
            };

            let allow_call_to_c = InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: call_c_on_behalf_of_a.clone(),
                sub_invocations: vec![&env],
            });

            // Authorize B to call C twice
            env.authorize_as_current_contract(vec![&env, allow_call_to_c.clone(), allow_call_to_c]);
            let client = ContractBClient::new(&env, &contract_b_address);
            client.call_d_twice(
                &env.current_contract_address(),
                &contract_c_address,
                &contract_d_address,
            );
        }

        pub fn auth_sub_invocation_bad_root(
            env: Env,
            contract_b_address: Address,
            contract_c_address: Address,
            contract_d_address: Address,
        ) {
            let call_c_on_behalf_of_a = ContractContext {
                contract: contract_c_address.clone(),
                fn_name: Symbol::new(&env, "authorized_fn_c"),
                args: (env.current_contract_address(),).into_val(&env),
            };
            let allow_call_to_c = InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: call_c_on_behalf_of_a,
                sub_invocations: vec![&env],
            });

            let none = ContractContext {
                contract: env.current_contract_address(),
                fn_name: Symbol::new(&env, ""),
                args: ().into_val(&env),
            };
            let allow_nested_call_to_c =
                InvokerContractAuthEntry::Contract(SubContractInvocation {
                    context: none,
                    sub_invocations: vec![&env, allow_call_to_c],
                });

            // Q: why doesn't this authorize B to call C?
            // A: because the root (none) doesn't match
            env.authorize_as_current_contract(vec![&env, allow_nested_call_to_c]);
            let client = ContractBClient::new(&env, &contract_b_address);
            log!(&env, "ContractA calling B::call_d()");
            client.call_d(
                &env.current_contract_address(),
                &contract_c_address,
                &contract_d_address,
            );
        }

        pub fn auth_sub_invocation(
            env: Env,
            contract_b_address: Address,
            contract_c_address: Address,
            contract_d_address: Address,
        ) {
            let call_c_on_behalf_of_a = ContractContext {
                contract: contract_c_address.clone(),
                fn_name: Symbol::new(&env, "authorized_fn_c"),
                args: (env.current_contract_address(),).into_val(&env),
            };
            let allow_call_to_c = InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: call_c_on_behalf_of_a,
                sub_invocations: vec![&env],
            });

            let call_d_on_behalf_of_a = ContractContext {
                contract: contract_d_address.clone(),
                fn_name: Symbol::new(&env, "call_c_authorized"),
                args: (
                    env.current_contract_address(),
                    contract_b_address.clone(),
                    contract_c_address.clone(),
                )
                    .into_val(&env),
            };
            let allow_nested_call_to_c =
                InvokerContractAuthEntry::Contract(SubContractInvocation {
                    context: call_d_on_behalf_of_a,
                    sub_invocations: vec![&env, allow_call_to_c],
                });

            // Q: why doesn't this authorize B to call C?
            // A: because the root (none) doesn't match
            env.authorize_as_current_contract(vec![&env, allow_nested_call_to_c]);
            let client = ContractBClient::new(&env, &contract_b_address);
            log!(&env, "ContractA calling B::call_authorized_d()");
            client.call_authorized_d(
                &env.current_contract_address(),
                &contract_c_address,
                &contract_d_address,
            );
        }
    }
}

mod contract_b {
    use soroban_sdk::{contract, contractimpl, log, Address, Env};

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

        pub fn call_d(
            env: Env,
            authorizer: Address,
            contract_c_address: Address,
            contract_d_address: Address,
        ) {
            log!(&env, "ContractB calling D::call_c()");
            authorizer.require_auth();
            let client = ContractDClient::new(&env, &contract_d_address);
            client.call_c(
                &authorizer,
                &env.current_contract_address(),
                &contract_c_address,
            );
        }

        pub fn call_authorized_d(
            env: Env,
            authorizer: Address,
            contract_c_address: Address,
            contract_d_address: Address,
        ) {
            log!(&env, "ContractB calling D::call_c()");
            authorizer.require_auth();
            let client = ContractDClient::new(&env, &contract_d_address);
            client.call_c_authorized(
                &authorizer,
                &env.current_contract_address(),
                &contract_c_address,
            );
        }

        pub fn call_d_twice(
            env: Env,
            authorizer: Address,
            contract_c_address: Address,
            contract_d_address: Address,
        ) {
            authorizer.require_auth();
            let client = ContractDClient::new(&env, &contract_d_address);
            log!(&env, "ContractB calling D::call_c()");
            client.call_c(
                &authorizer,
                &env.current_contract_address(),
                &contract_c_address,
            );
            // Direct call to C
            let client = ContractCClient::new(&env, &contract_c_address);
            log!(&env, "ContractB calling C::authorized_fn_c()");
            client.authorized_fn_c(&authorizer);
        }
    }
}
mod contract_c {

    use soroban_sdk::{contract, contractimpl, log, Address, Env};

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

    use soroban_sdk::{contract, contractimpl, log, Address, Env};

    use crate::contract_c::ContractCClient;

    #[contract]
    pub struct ContractD;

    #[contractimpl]
    impl ContractD {
        pub fn call_c(
            env: Env,
            authorizer: Address,
            contract_b_address: Address,
            contract_c_address: Address,
        ) {
            contract_b_address.require_auth(); // this should always succeed when called by B
            log!(&env, "ContractD calling C::authorized_fn_c()");
            let client = ContractCClient::new(&env, &contract_c_address);
            client.authorized_fn_c(&authorizer);
        }

        pub fn call_c_authorized(
            env: Env,
            authorizer: Address,
            contract_b_address: Address,
            contract_c_address: Address,
        ) {
            contract_b_address.require_auth(); // this should always succeed when called by B
            authorizer.require_auth();
            log!(&env, "ContractD calling C::authorized_fn_c()");
            let client = ContractCClient::new(&env, &contract_c_address);
            client.authorized_fn_c(&authorizer);
        }
    }
}

mod test;
