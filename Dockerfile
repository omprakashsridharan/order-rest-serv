FROM lukemathwalker/cargo-chef:0.1.39-rust-latest AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/orderserv /usr/local/bin/orderserv
COPY ./src/lib/config /usr/local/bin/config
ENV WAIT_VERSION 2.7.2
ADD https://github.com/ufoscout/docker-compose-wait/releases/download/$WAIT_VERSION/wait /usr/local/bin/wait
RUN chmod +x /usr/local/bin/wait
ENTRYPOINT ["/usr/local/bin/orderserv"]
