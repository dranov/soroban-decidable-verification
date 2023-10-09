ARG FROM_IMAGE_FOR_SPECTOOLS
FROM ${FROM_IMAGE_FOR_SPECTOOLS}

ARG USERNAME
USER ${USERNAME}

# mypyvy
WORKDIR ${USER_HOME}
RUN git clone https://github.com/wilcoxjay/mypyvy.git \
    && cd mypyvy \
    && pip3 install -r requirements.txt \
    && pip3 install z3-solver

# temporal-verifier / flyvy
WORKDIR ${USER_HOME}
RUN . $HOME/.cargo/env \ 
    && git clone https://github.com/vmware-research/temporal-verifier.git \
    && cd temporal-verifier \
    && ./tools/download-solvers.sh \
    && cargo build --release

# ivy (Python 3)
WORKDIR ${USER_HOME}
RUN sudo apt-get install --no-install-recommends --yes \
    g++ cmake python3-ply python3-tk tix libssl-dev python3-setuptools \
    libreadline-dev python3-pyparsing python-is-python3

RUN git clone https://github.com/nano-o/ivy.git \
    && cd ivy \
    && git checkout 2to3new \
    && git submodule update --init \
    && python3 build_submodules.py

RUN pip3 install tarjan pydot
# ENV PYTHONPATH=/home/${USERNAME}/ivy/:/home/${USERNAME}/ivy/ivy/

