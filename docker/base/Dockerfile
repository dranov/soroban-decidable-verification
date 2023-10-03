ARG UBUNTU_VERSION
FROM ubuntu:${UBUNTU_VERSION}

# Install dependencies
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get --yes update \
  && apt-get install --no-install-recommends --yes \
  bison \
  bubblewrap \
  build-essential \
  bzip2 \
  ca-certificates \
  clang \
  cmake \
  curl \
  flex \
  gcc \
  git \
  libboost-all-dev \ 
  libgmp-dev \
  make \
  patch \
  perl \
  pkg-config \
  python3 \
  rsync \
  sudo \
  unzip \
  wget \
  # Cleanup
  && apt-get clean

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install Opam and OCaml
RUN curl -fsSL https://raw.githubusercontent.com/ocaml/opam/master/shell/install.sh -o /tmp/install.sh \
  && yes "" | sh /tmp/install.sh \
  && rm /tmp/install.sh

# Placeholder args that are expected to be passed in at image build time.
# See https://code.visualstudio.com/docs/remote/containers-advanced#_creating-a-nonroot-user
ARG USERNAME=ar
ARG USER_UID=1000
ARG USER_GID=${USER_UID}
ENV USER_HOME=/home/${USERNAME}

# Create the specified user and group and add them to sudoers list
#
# Ignore errors if the user or group already exist (it should only happen if the image is being
# built as root, which happens on GCB).
RUN (groupadd --gid=${USER_GID} ${USERNAME} || true) \
  && (useradd --shell=/bin/bash --uid=${USER_UID} --gid=${USER_GID} --create-home ${USERNAME} || true) \
  && echo "${USERNAME}  ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers



