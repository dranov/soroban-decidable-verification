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

RUN cd ivy \
    && pip2 install . 

# RUN sudo rm /usr/bin/python
RUN pip2 install pyparsing ply tarjan pydot z3-solver
RUN sudo apt-get install -y graphviz && sudo apt-get clean

# ivy (Python 3)
# WARNING: we saw some errors in generated traces with this version!
# WORKDIR ${USER_HOME}

RUN sudo apt-get install -y python3-pip python3-tk graphviz && sudo apt-get clean
RUN pip3 install pyparsing ply tarjan pydot z3-solver

RUN git clone https://github.com/dranov/ivy.git ivy3 \
    && cd ivy3 \
    && git checkout mypyvy-translation \
    && git submodule update --init

# For some reason, the Python 3 version of Ivy needs the built Z3
# rather than being okay with the z3-solver package
RUN cd ivy3 \
    && sudo ln -s /usr/bin/python3 /usr/bin/python \
    && python3 build_submodules.py \
    && sudo rm /usr/bin/python

RUN cd ivy3 \
    && sudo pip3 install .

# # Install Duo
# RUN git clone https://github.com/VeriGu/Duo.git \
#     && cd Duo \
#     && sed -i 's$#define IVY_CHECK_PATH "/home/me/anaconda3/envs/py2/bin/ivy_check"$#define IVY_CHECK_PATH "/usr/local/bin/ivy_check"$g' src-c/InvRefiner.h \
#     && cd src-c \
#     && make
# RUN sudo apt-get install -y python3-pip
# RUN pip3 install numpy scipy pandas

# RUN git clone https://github.com/dranov/ivy.git \
#     && cd ivy \
#     && git checkout 2to3new \
#     && git submodule update --init \
#     && python3 build_submodules.py \
#     && sudo python3 setup.py install

# RUN pip3 install pyparsing ply tarjan pydot
# RUN sudo apt-get install -y graphviz && sudo apt-get clean
# ENV PYTHONPATH=/home/${USERNAME}/ivy/:/home/${USERNAME}/ivy/ivy/

