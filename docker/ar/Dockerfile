ARG FROM_IMAGE_FOR_AR
FROM ${FROM_IMAGE_FOR_AR}

ARG USERNAME
USER ${USERNAME}

# RUN . $HOME/.cargo/env \
#     && rustup default nightly \
#     && rustup component add miri

# Soroban
# RUN . $HOME/.cargo/env \
#     && rustup target add wasm32-unknown-unknown
# RUN . $HOME/.cargo/env \
#     && cargo install --locked --version 20.0.0-rc2 soroban-cli
# RUN echo "source <(soroban completion --shell bash)" >> /home/${USERNAME}/.bashrc

RUN echo "DONE"