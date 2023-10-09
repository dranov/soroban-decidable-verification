ARG FROM_IMAGE_FOR_VERIFIERS
FROM ${FROM_IMAGE_FOR_VERIFIERS}

ARG USERNAME
USER ${USERNAME}
WORKDIR ${USER_HOME}

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Kani
RUN sudo apt-get install -y python3-pip
RUN . $HOME/.cargo/env \
    && cargo install --locked kani-verifier \
    && cargo kani setup

# Creusot
RUN sudo apt-get install --yes libcairo2-dev libgtk-3-dev libgtksourceview-3.0-dev
RUN opam pin add --yes why3 https://gitlab.inria.fr/why3/why3.git \
    && opam pin add --yes why3-ide https://gitlab.inria.fr/why3/why3.git \
    && opam update

RUN opam install --yes lablgtk3 lablgtk3-sourceview3 ocamlgraph why3 why3-ide
RUN eval $(opam env) && why3 config detect
RUN sudo apt-get -y install libssl-dev
RUN . $HOME/.cargo/env \
    && git clone https://github.com/xldenis/creusot.git \
    && cd creusot \
    && cargo install --path cargo-creusot
    # TODO: need to run and persist
    # export LD_LIBRARY_PATH+=:$(rustc --print=sysroot)/lib

# # Prusti: https://github.com/viperproject/prusti-dev/blob/master/Dockerfile
ARG PRUSTI_VERSION
RUN DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC sudo -E apt-get -y install tzdata
RUN sudo apt-get install --yes default-jdk mono-complete libssl-dev \
    && sudo apt-get clean

WORKDIR ${USER_HOME}
RUN wget -nv -O prusti.zip https://github.com/viperproject/prusti-dev/releases/download/v-${PRUSTI_VERSION}/prusti-release-ubuntu.zip \
    && unzip -q prusti.zip -d prusti \
    && rm prusti.zip \
    && chmod +x -R prusti \
    && sudo mv prusti /usr/local/prusti

RUN . $HOME/.cargo/env \
    && CHANNEL=$(cat /usr/local/prusti/rust-toolchain | grep channel | cut -d'"' -f2) \
    && rustup install "$CHANNEL"
ENV PATH /usr/local/prusti:$PATH

# Flux
WORKDIR ${USER_HOME}

RUN curl -sSL https://get.haskellstack.org/ | sudo sh
RUN git clone https://github.com/ucsd-progsys/liquid-fixpoint.git \
    && cd liquid-fixpoint \
    && stack install

WORKDIR ${USER_HOME}
RUN . $HOME/.cargo/env \
    && git clone https://github.com/liquid-rust/flux \
    && cd flux \
    && cargo xtask install

# MIRAI
WORKDIR ${USER_HOME}
RUN . $HOME/.cargo/env \
    && git clone https://github.com/facebookexperimental/MIRAI.git \
    && cd MIRAI \
    && cargo install --locked --path ./checker

# Aeneas
RUN opam switch create 4.13.1+options
RUN opam install --yes ppx_deriving visitors easy_logging zarith yojson core_unix odoc \
    unionFind ocamlgraph
RUN . $HOME/.cargo/env && eval $(opam env) \
    git clone https://github.com/AeneasVerif/charon.git \
    && cd charon \
    && make \
    && cd charon-ml \
    && opam install --yes .

RUN . $HOME/.cargo/env && eval $(opam env) \
    && git clone https://github.com/AeneasVerif/aeneas.git \
    && cd aeneas \
    && ln -s ~/charon/charon-ml charon \ 
    && make

# Crucible & Crux MIR
RUN curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | BOOTSTRAP_HASKELL_NONINTERACTIVE=1 sh

RUN . $HOME/.ghcup/env && . $HOME/.cargo/env \
    && git clone https://github.com/GaloisInc/crucible.git \
    && cd crucible \
    && git submodule update --init \
    && cd ./dependencies/mir-json \
    && CHANNEL=$(cat ./rust-toolchain.toml | grep channel | cut -d'"' -f2) \
    && rustup toolchain install "$CHANNEL" --force \
    && rustup component add --toolchain "$CHANNEL" rustc-dev \
    && cargo install --locked

RUN . $HOME/.ghcup/env && . $HOME/.cargo/env \
    && cd crucible/crux-mir \
    && git submodule update --init \
    && cabal v2-install exe:crux-mir --overwrite-policy=always

RUN . $HOME/.ghcup/env && . $HOME/.cargo/env \
    && cd crucible/crux-mir \
    && ./translate_libs.sh

# TODO: add SeaHorn