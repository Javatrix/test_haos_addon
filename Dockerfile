ARG BUILD_FROM
FROM $BUILD_FROM

# Install rustup and the Rust toolchain
RUN apk add --no-cache rustup && \
    rustup-init -y --no-modify-path && \
    source $HOME/.cargo/env && \
    rustup default stable

WORKDIR /data

# Copy data for add-on
COPY src /
COPY Cargo.toml /
COPY Cargo.lock /

CMD ["/root/.cargo/bin/cargo", "run"]
