# Base image with Rust and pre-installed SSL packages
FROM rust:1.44.1 AS base
WORKDIR /app

# Necessary tools (for the ICMP packages)
RUN apt-get update && \
    apt-get install libssl-dev

# Build stage, install deps and build the CLI app
FROM base AS build

# Install Rust crates only (use dummy main.rs)
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src && echo "fn main() {println!(\"NOT THE APP!\")}" > src/main.rs
RUN cargo build --release && rm src/main.rs

# Build the sources
COPY src/ src/
RUN cargo build --release

# Final release image
FROM alpine:latest as latest
WORKDIR /app
COPY --from=build /app/target/release/yant .