# soroban-decidable-verification

Experiments with verifying Soroban smart contracts using decidable
logic

## Docker Image

We have a Docker image with a number of verification tools installed:

```bash
sudo service docker start
./docker/build
```

### Creusot
```bash
cd ~/creusot && export LD_LIBRARY_PATH+=:$(rustc --print=sysroot)/lib
```

### Prusti

```bash
. $HOME/.cargo/env \
&& CHANNEL=$(cat /usr/local/prusti/rust-toolchain | grep channel | cut -d'"' -f2) \
&& rustup install "$CHANNEL"
export PATH=/usr/local/prusti:$PATH
```

### Ivy

To run Ivy correctly, do not invoke the scripts directly, but rather
[run them through the module system](https://stackoverflow.com/a/65589847), e.g.
`python3 -m ivy.ivy_check examples/ivy/array.ivy`. In the Docker image, you can
just:

```bash
ivy_check examples/ivy/array.ivy
ivy_show isolate=this models/token.ivy
```

## Relevant Tools

### Rust verification

- [aeneas](https://github.com/AeneasVerif/aeneas) -- Rust to F*, Coq, HOL4, and Lean
- [bolero](https://github.com/camshaft/bolero) -- property testing and verification frontend
- [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) -- fuzzing
- [cargo-verify](https://github.com/project-oak/rust-verification-tools/tree/main/cargo-verify) -- unmaintained
- [coq-of-rust](https://github.com/formal-land/coq-of-rust) -- Rust to Coq
- [creusot](https://github.com/xldenis/creusot) -- deductive verification by translation to Why3 (and then SMT solvers)
- [crucible](https://github.com/GaloisInc/crucible) -- symbolic execution ([crux-mir](https://github.com/GaloisInc/crucible/tree/master/crux-mir))
- [flux](https://github.com/flux-rs/flux) -- refinement types for Rust
- [hacspec-v2](https://github.com/hacspec/hacspec-v2) -- Rust to Coq/F*
- [kani](https://github.com/model-checking/kani) -- bit-precise model checker
- [mirai](https://github.com/facebookexperimental/MIRAI) -- abstract interpreter for MIR
- [miri](https://github.com/rust-lang/miri) -- instrumented interpreter
- [proptest](https://github.com/proptest-rs/proptest) -- property-based testing
- [prusti](https://github.com/viperproject/prusti-dev) -- static verifier based on [Viper](https://www.pm.inf.ethz.ch/research/viper.html)
- [rudra](https://github.com/sslab-gatech/Rudra) -- memory safety & undefined behaviour detection
- [rust-horn](https://github.com/hopv/rust-horn) -- CHC-based automated verifier
- [seahorn](https://github.com/seahorn/seahorn) -- automated analysis framework for LLVM-based languages
- [verus](https://github.com/verus-lang/verus) -- subset of Rust with support for verification
