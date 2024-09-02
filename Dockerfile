FROM rust:latest

# Install JSON parser for options.json
RUN apt-get update && apt-get install -y jq bash

WORKDIR /build

# Copy files
COPY . .

RUN chmod +x /build/run.sh
# Build the Rust project
RUN cargo build --release

EXPOSE 9128

# Run the compiled binary
ENTRYPOINT ["./run.sh"]
