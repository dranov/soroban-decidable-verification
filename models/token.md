# Token contract model

Storage types:

```Rust
pub enum DataKey {
    Allowance(AllowanceDataKey),
    Balance(Address),
    Nonce(Address),
    State(Address),
    Admin,
}
```

- Admin module: `has`, `read`, `write` administrator
- Allowance module: `read`, `write`, `spend`
	- `AllowanceDataKey(from, spender)`
	- `amount: i128`, `expiration_ledger: u32`
	- allowances are stored in temporary storage
- Balance module: `read`, `write`, `receive`, `spend`
	- balances stored in persistent storage (i.e. can be restored after expiry)
- Metadata module - can probably ignore
- Contract:
	- `initialize` - can be called by anyone if not already initialized
	- `mint`

We also need to model:
 - the environment, e.g. `e.ledger().sequence()``
 - Soroban's host authorization logic
 - the panic/abort system
 
## Observations

- Because Soroban has the catch-all `Val` type, we have to be careful to
  disambiguate what is actually stored in the ledger mappings. The `get`
  operation requires a type annotation in Soroban contracts, e.g.
  `get::<DataKey, i128>`.



## Step-by-step modelling process

1. Identify data types:
    - `Address`
    - `AllowanceValue { amount: i128, expiration_ledger: u32}`
    - 