# Stage 1: Build the Rust application
FROM rust:latest as builder

WORKDIR /usr/src/myapp

# Copy dependencies and build
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the rest of the source code and build again
COPY . .
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:stable-slim

WORKDIR /usr/local/bin

# Copy the compiled binary from the build stage
COPY --from=builder /usr/src/myapp/target/release/rust_addon .

# Run the binary
CMD ["./rust_addon"]
