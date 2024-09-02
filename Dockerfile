FROM rust:latest

WORKDIR /build

# Copy files
COPY . .

# Build the Rust project
RUN cargo build --release

# Run the compiled binary
CMD ["./target/release/test_addon"]
