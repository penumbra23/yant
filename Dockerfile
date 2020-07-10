# Base image with Rust and pre-installed SSL packages
FROM ekidd/rust-musl-builder AS base
WORKDIR /home/rust/src

USER rust

# Build stage, install deps and build the CLI app
FROM base AS build

# Copy crates
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Build the sources
COPY src/ src/
RUN cargo build --release

# Final release image
FROM alpine:latest as latest
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/yant /usr/local/bin/yant