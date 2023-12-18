# soroban-decidable-verification

Experiments with verifying Soroban smart contracts using decidable logic. These
are research prototypes. Use them at your own risk; no warranty is provided!

We offer a framework for reasoning about Soroban smart contracts using
[Ivy](https://github.com/kenmcmil/ivy), a research tool for automation-assisted
interactive development and verification of protocols.

You can create a _model_ of your Soroban smart contract, state the desired
safety properties, and then with the help of Ivy, interactively discover an
inductive invariant that establishes the desired properties:

- translating your contract to Ivy is relatively straightforward, especially if
  the contract does not use complex data structures;
- discovering the inductive invariant can be a laborious process, requiring some
  expertise.

A tutorial for Ivy can be found in the
[`doc/`](https://github.com/kenmcmil/ivy/tree/master/doc) folder in the Ivy
repository.


## Docker Image

We have a Docker image with a number of verification tools installed, including
our custom versions of
[Ivy](https://github.com/dranov/ivy/tree/soroban-improvements) and
[mypyvy](https://github.com/dranov/mypyvy/commits/trace-dump):

```bash
sudo service docker start
./docker/build
./docker/run
```

### Examples

(1) To check all invariants on a given specification:

```bash
ivy_check token.ivy
```

(2) If an invariant fails to be maintained, you can obtain a concrete
counter-example:

```bash
ivy_check trace=true token.ivy
```

This prints a counter-example to induction (CTI) in the form of a pre-state and
step-by-step execution of the relevant transition.

(3) If you do not care about the step-by-step execution, you can greatly speed
up the time it takes to generate the counter-example, you can run:

```bash
ivy_check model=true shrink=false token.ivy
```

This generates only the pre-state and tells Ivy to not minimize the generated model. Alternatively, if you want only the beginning of the step-by-step execution (e.g., to figure out which transition arguments we used), you can run:


```bash
ivy_check trace=true action_depth=0 token.ivy
```

`action_depth` can be any integer. Larger integers print more of the execution steps.

(4) To convert an Ivy specification to mypyvy, add `attribute method=convert_to_mypyvy` in the `.ivy` file and then run `ivy_check`.

If you want to use SMT queries to simplify the resulting mypyvy file, call
`ivy_check simplify=true`. This might take minutes to hours on larger
specifications, but should make queries on the mypyvy side much faster,
especially for invariant inference.

 You can then call `mypyvy` on the resulting `.pyv` file, e.g.:

```bash
mypyvy verify token.ivy
mypyvy updr token.ivy
```


## Specification walkthrough

This repository contains Ivy specifications for:

- an abstracted Soroban environment (`soroban.ivy`)
- abstracted versions of the integers (`integers.ivy`)
- a model of the Token contract (`token_contract.ivy`)
- a model of the Liquidity Pool contract (`liquidity_contract.ivy`)

### Soroban environment

We have modeled a bare-bones version of the Soroban environment, which provides
basic support for reasoning about (a) authorization properties and (b) whether
transactions panic.

The environment includes a `partial_map` module to represent mappings (key-value
dictionaries) and two modules that abstract the integers: `simplified_integer`
(addition, substraction, comparison) and `decidable_integer` (multiplication,
division, sqrt). It does NOT model expiration for temporary storage – all storage is modeled as permanent.

## Tips

- Remove constants and relations that are not needed in your specification (e.g.
  `minus_one` or ghost state), as each extra constant/relation can exponentially
  increase solver times
