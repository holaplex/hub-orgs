FROM rust:1.58.1-slim-bullseye AS build
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

COPY ./src ./src
COPY ./migration ./migration
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release


FROM debian:bullseye-slim
WORKDIR /hug-orgs
RUN apt-get update -y && \
  apt-get install -y \
    ca-certificates \
    libpq5 \
    libssl1.1 \
  && \
  rm -rf /var/lib/apt/lists/*

COPY .env ./
COPY --from=build /build/target/release/hub-orgs /hub-orgs
