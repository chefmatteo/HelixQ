# syntax=docker/dockerfile:1.7

FROM rust:1-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --locked --release && \
    cp target/release/HelixQ /usr/local/bin/helixq

FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install --yes --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --uid 10001 helixq
USER helixq
WORKDIR /home/helixq

COPY --from=builder /usr/local/bin/helixq /usr/local/bin/helixq

ENTRYPOINT ["helixq"]
CMD ["--help"]
