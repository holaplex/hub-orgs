FROM rust:1.58.1-slim-bullseye AS chef
RUN cargo install cargo-chef 
WORKDIR /build


RUN apt-get update -y && \
  apt-get install -y \
    libpq-dev \
    libssl-dev \
    libudev-dev \
    pkg-config \
  && \
  rm -rf /var/lib/apt/lists/*

COPY rust-toolchain.toml ./

# Force rustup to install toolchain
RUN rustc --version

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --profile docker --bin migrator \ 
--bin graphql


FROM debian:bullseye-slim as base
WORKDIR /hug-orgs
RUN apt-get update -y && \
  apt-get install -y \
    ca-certificates \
    libpq5 \
    libssl1.1 \
  && \
  rm -rf /var/lib/apt/lists/*

RUN mkdir -p bin

FROM base as migrator
COPY --from=builder /build/target/release/migrator bin/

FROM base as graphql
COPY --from=builder /build/target/release/graphql bin/
