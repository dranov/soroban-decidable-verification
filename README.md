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