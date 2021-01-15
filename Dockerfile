FROM rust:1-slim as builder

WORKDIR /usr/src/traefik-pages

COPY ./src /usr/src/traefik-pages/src
COPY Cargo.toml /usr/src/traefik-pages/Cargo.toml
COPY Cargo.lock /usr/src/traefik-pages/Cargo.lock

RUN cargo build --release

# Runtime
FROM debian:buster-slim

COPY --from=builder /usr/src/traefik-pages/target/release/traefik-pages /usr/local/bin/traefik-pages

ADD https://github.com/krallin/tini/releases/download/v0.19.0/tini /tini
RUN chmod +x /tini

USER www-data

ENV PORT 5000
EXPOSE 5000

ENTRYPOINT ["/tini", "--"]
CMD ["/usr/local/bin/traefik-pages"]
