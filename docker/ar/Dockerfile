ARG FROM_IMAGE_FOR_AR
FROM ${FROM_IMAGE_FOR_AR}

RUN . $HOME/.cargo/env \
    && rustup default nightly \
    && rustup component add miri