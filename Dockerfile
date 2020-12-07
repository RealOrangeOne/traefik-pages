FROM rust:1-slim as builder

WORKDIR /usr/src/traefik-pages

COPY ./src /usr/src/traefik-pages/src
COPY Cargo.toml /usr/src/traefik-pages/Cargo.toml
COPY Cargo.lock /usr/src/traefik-pages/Cargo.lock

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/traefik-pages/target/release/traefik-pages /usr/local/bin/traefik-pages

ENTRYPOINT "/usr/local/bin/traefik-pages"
