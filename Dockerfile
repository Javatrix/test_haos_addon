ARG BUILD_FROM
FROM $BUILD_FROM

# Install rustup and the Rust toolchain
RUN apk add --no-cache rustup && \
    rustup-init -y --no-modify-path && \
    source $HOME/.cargo/env && \
    rustup default stable

WORKDIR /build

# Copy source code and Cargo files
COPY Cargo.toml Cargo.lock /build/
COPY src/ /build/src/

CMD ["ls", "/build", "&&", "cargo", "run"]
