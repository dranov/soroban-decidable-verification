ARG FROM_IMAGE_FOR_SPECTOOLS
FROM ${FROM_IMAGE_FOR_SPECTOOLS}

ARG USERNAME
USER ${USERNAME}

# mypyvy
RUN sudo apt-get update \
    && sudo apt-get install -y python3-pip \
    && sudo apt-get clean
WORKDIR ${USER_HOME}
RUN git clone https://github.com/dranov/mypyvy.git\
    && cd mypyvy \
    && git checkout trace-dump \
    && pip3 install -r requirements.txt \
    && pip3 install z3-solver

# temporal-verifier / flyvy
WORKDIR ${USER_HOME}
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . $HOME/.cargo/env \ 
    && git clone https://github.com/vmware-research/temporal-verifier.git \
    && cd temporal-verifier \
    && ./tools/download-solvers.sh \
    && cargo build --release

# ivy (Python 3)
WORKDIR ${USER_HOME}
RUN DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC sudo -E apt-get -y install tzdata
RUN sudo apt-get install -y \
    g++ cmake tix libssl-dev libreadline-dev \
    python3-pip python3-tk python-is-python3 graphviz \
    && sudo apt-get clean
RUN pip3 install pyparsing ply tarjan pydot z3-solver

RUN git clone https://github.com/dranov/ivy.git \
    && cd ivy \
    && git checkout soroban-improvements \
    && git submodule update --init

# For some reason, the Python 3 version of Ivy needs the built Z3
# rather than being okay with the z3-solver package
RUN cd ivy \
    && python3 build_submodules.py
RUN cd ivy \
    && sudo pip3 install .

# # Install Duo
RUN git clone https://github.com/VeriGu/Duo.git \
    && cd Duo \
    && sed -i 's$#define IVY_CHECK_PATH "/home/me/anaconda3/envs/py2/bin/ivy_check"$#define IVY_CHECK_PATH "/usr/local/bin/ivy_check"$g' src-c/InvRefiner.h \
    && cd src-c \
    && make
RUN sudo apt-get install -y python3-pip
RUN pip3 install numpy scipy pandas

# Add links to make calling tools easier
RUN sudo ln -s ${USER_HOME}/mypyvy/src/mypyvy.py /usr/local/bin/mypyvy
RUN sudo ln -s ${USER_HOME}/temporal-verifier/target/release/temporal-verifier /usr/local/bin/flyvy
