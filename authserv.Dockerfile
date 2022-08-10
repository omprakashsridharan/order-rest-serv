# FROM lukemathwalker/cargo-chef:latest AS chef
# WORKDIR app

# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM chef AS builder 
# COPY --from=planner /app/recipe.json recipe.json
# # Build dependencies - this is the caching Docker layer!
# RUN cargo chef cook --release --recipe-path recipe.json
# # Build application
# COPY . .
# RUN cargo build --release --package authserv

# # We do not need the Rust toolchain to run the binary!
# FROM debian:buster-slim AS runtime
# WORKDIR app
# COPY --from=builder /app/target/release/authserv /usr/local/bin

# ENV WAIT_VERSION 2.7.2
# ADD https://github.com/ufoscout/docker-compose-wait/releases/download/$WAIT_VERSION/wait /usr/local/bin/wait
# RUN chmod +x /usr/local/bin/wait

# CMD ["/usr/local/bin/authserv"]

ARG BASE_IMAGE=rust:latest

FROM $BASE_IMAGE as planner
WORKDIR /app
RUN cargo install cargo-chef --version 0.1.39
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM $BASE_IMAGE as  cacher
WORKDIR /app
RUN cargo install cargo-chef --version 0.1.39
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer! Don't build in release mode for development
RUN cargo chef cook -p authserv --release --recipe-path recipe.json 


FROM $BASE_IMAGE as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --bin authserv

FROM alpine AS authserv
WORKDIR /app
# We are building in debug mode
COPY --from=builder /app/target/release/authserv /usr/local/bin
ENV WAIT_VERSION 2.7.2
ADD https://github.com/ufoscout/docker-compose-wait/releases/download/$WAIT_VERSION/wait /usr/local/bin/wait
RUN chmod +x /usr/local/bin/wait
ENTRYPOINT ["/usr/local/bin/authserv"]