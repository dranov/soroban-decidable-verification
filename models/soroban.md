# Soroban Environment Model

## Transactions

Intuitively, we ought to account for the transactional nature of Soroban smart
contracts, i.e. contract transitions/methods are not called "by themselves", but
only within the context of a transaction, and the blockchain environment *does
not* change within the context of a transaction.

### Panics

In this context, panics can be modelled as a flag that is set when `panic!` is
called. In the `end_transaction()` Ivy transition, if the `panic` flag is set,
then all changes are aborted.

## Authorization

### Implementation
Authorization is described in
[CAP 0046-11 Soroban Authorization Framework](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0046-11.md)
and implemented in `rs-soroban-env/soroban-env-host/src/auth.rs`. Tests are in
`rs-soroban-env/soroban-env-host/src/test/auth.rs` and
`rs-soroban-sdk/tests/auth/src/lib.rs`.

The transaction contains an `auth_entries` vector of `SorobanAuthorizationEntry`
records. In each entry, an `address` (either "user" or "contract" address)
authorizes a tree of invocations, consisting of a `root_invocation` and
`sub_invocations`.

The call stack at runtime of Soroban functions that call `require_auth` must be
a sub-tree (i.e., connected from the root, without "gaps", but with potentially
missing nodes, including all their children) of the authorization tree.

Authorization is managed by the `AuthorizationManager` data structure, created
in `new_enforcing()` from the vector of `auth_entries`. The entry-point for
authorization calls is `require_auth_enforcing()`

```Rust
// Authorization manager encapsulates host-based authentication & authorization
// framework.
pub struct AuthorizationManager {
    // Per-address trackers of authorized invocations.
    // Every tracker takes care about a single rooted invocation tree for some
    // address. There can be multiple trackers per address.
    account_trackers: RefCell<Vec<RefCell<AccountAuthorizationTracker>>>,
    // Per-address trackers for authorization performed by the contracts at
    // execution time (as opposed to signature-based authorization for accounts).
    invoker_contract_trackers: RefCell<Vec<InvokerContractAuthorizationTracker>>,
    // Current call stack consisting only of the contract invocations (i.e. not
    // the host functions).
    call_stack: RefCell<Vec<AuthStackFrame>>,
}
```

The core idea is that each `AccountAuthorizationTracker` has an
`InvocationTracker` that maintains the current walk in the tree of authorized
invocations for the given authorizer `address`.

When a `require_auth()` call happens, `require_auth_enforcing()` is called, and
finds _the first_ tracker for the authorizer (matching address) that can
authorize the current call. If a tracker is "made active" (selected), you cannot
go into other trackers to match sub-trees of the call stack, i.e. you can't pick
and choose between trackers/authorization trees -- you must commit to one.

### Model

We want to be able to state properties of the form "if X happens, then Y
authorization must have been given" (i.e. "X cannot happen without Y
authorization").
