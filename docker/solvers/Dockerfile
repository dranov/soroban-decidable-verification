ARG FROM_IMAGE_FOR_SOLVERS
FROM ${FROM_IMAGE_FOR_SOLVERS}

ARG USERNAME
USER ${USERNAME}

# Install minisat solver

RUN mkdir ${USER_HOME}/minisat
WORKDIR ${USER_HOME}/minisat

ARG MINISAT_VERSION
RUN git clone --no-checkout https://github.com/stp/minisat.git \
  && cd minisat \
  && git checkout ${MINISAT_VERSION} \
  && git submodule init \
  && git submodule update \
  && mkdir build \
  && cd build \
  && cmake .. \
  && make -j4 \
  && sudo make install \
  && make clean

# Install yices solver

RUN sudo apt-get install --yes software-properties-common \
    && sudo add-apt-repository ppa:sri-csl/formal-methods \
    && sudo apt-get update \
    && sudo apt-get install --yes yices2

# Install CVC4 solver
ARG CVC4_VERSION
RUN wget -nv -O cvc4 https://github.com/CVC4/CVC4/releases/download/${CVC4_VERSION}/cvc4-${CVC4_VERSION}-x86_64-linux-opt \
    && chmod +x cvc4 \
    && sudo mv cvc4 /usr/bin

# Install CVC5 solver
ARG CVC5_VERSION
RUN wget -nv -O cvc5 https://github.com/cvc5/cvc5/releases/download/cvc5-${CVC5_VERSION}/cvc5-Linux \
    && chmod +x cvc5 \
    && sudo mv cvc5 /usr/bin

# Install Z3
RUN mkdir ${USER_HOME}/z3
WORKDIR ${USER_HOME}/z3
ARG Z3_VERSION
RUN curl --location https://github.com/Z3Prover/z3/releases/download/z3-${Z3_VERSION}/z3-${Z3_VERSION}-x64-glibc-2.31.zip > z3.zip \
  && unzip -q z3.zip \
  && rm z3.zip \
  && sudo mv z3-${Z3_VERSION}-x64-glibc-2.31/bin/z3 /usr/bin

# Install Alt-Ergo
RUN opam init --disable-sandboxing
RUN opam install --yes alt-ergo