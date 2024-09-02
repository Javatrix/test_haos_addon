FROM rust:latest

# Install JSON parser for options.json
RUN apk add --no-cache jq

WORKDIR /build

# Copy files
COPY . .

RUN chmod +x /build/run.sh
# Build the Rust project
RUN cargo build --release

# Run the compiled binary
ENTRYPOINT ["./run.sh"]
