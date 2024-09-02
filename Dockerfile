ARG BUILD_FROM
FROM $BUILD_FROM

# Install requirements for add-on
RUN \
  apk add --no-cache \
    rustup

WORKDIR /data

# Copy data for add-on
COPY src /
COPY Cargo.toml /
COPY Cargo.lock /

CMD [ "cargo run" ]
