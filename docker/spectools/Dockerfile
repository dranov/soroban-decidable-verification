ARG FROM_IMAGE_FOR_SPECTOOLS
FROM ${FROM_IMAGE_FOR_SPECTOOLS}

ARG USERNAME
USER ${USERNAME}

# mypyvy
RUN sudo apt-get update \
    && sudo apt-get install -y python3-pip \
    && sudo apt-get clean
WORKDIR ${USER_HOME}
RUN git clone https://github.com/wilcoxjay/mypyvy.git \
    && cd mypyvy \
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

# ivy (Python 2)
WORKDIR ${USER_HOME}
RUN DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC sudo -E apt-get -y install tzdata
RUN sudo apt-get install --no-install-recommends --yes \
    g++ cmake tix libssl-dev libreadline-dev \
    python2 python-pip python-tk  python-setuptools  \
    && sudo apt-get clean

RUN git clone https://github.com/dranov/ivy.git \
    && cd ivy \
    && git checkout 648e6b334ba1d2eaaf53bd62dc639d41aa794b0f \
    && git submodule update --init

RUN sudo ln -s /usr/bin/python2 /usr/bin/python \
    && cd ivy \
    && python build_submodules.py \
    && sudo python setup.py install 

# RUN sudo rm /usr/bin/python
RUN pip2 install pyparsing ply tarjan pydot
RUN sudo apt-get install -y graphviz && sudo apt-get clean

# Install Duo
RUN git clone https://github.com/VeriGu/Duo.git \
    && cd Duo \
    && sed -i 's$#define IVY_CHECK_PATH "/home/me/anaconda3/envs/py2/bin/ivy_check"$#define IVY_CHECK_PATH "/usr/local/bin/ivy_check"$g' src-c/InvRefiner.h \
    && cd src-c \
    && make
RUN sudo apt-get install -y python3-pip
RUN pip3 install numpy scipy pandas

# RUN git clone https://github.com/dranov/ivy.git \
#     && cd ivy \
#     && git checkout 2to3new \
#     && git submodule update --init \
#     && python3 build_submodules.py \
#     && sudo python3 setup.py install

# RUN pip3 install pyparsing ply tarjan pydot
# RUN sudo apt-get install -y graphviz && sudo apt-get clean

# ivy (Python 3)
# WORKDIR ${USER_HOME}
# RUN DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC sudo -E apt-get -y install tzdata
# RUN sudo apt-get install --no-install-recommends --yes \
#     g++ cmake python3-ply python3-tk tix libssl-dev python3-setuptools \
#     libreadline-dev python3-pyparsing python-is-python3 \
#     && sudo apt-get clean

# RUN git clone https://github.com/dranov/ivy.git \
#     && cd ivy \
#     && git checkout 2to3new \
#     && git submodule update --init \
#     && python3 build_submodules.py \
#     && sudo python3 setup.py install

# RUN pip3 install pyparsing ply tarjan pydot
# RUN sudo apt-get install -y graphviz && sudo apt-get clean
# ENV PYTHONPATH=/home/${USERNAME}/ivy/:/home/${USERNAME}/ivy/ivy/

