FROM rust:1-slim as builder

WORKDIR /usr/src/traefik-pages

COPY ./src /usr/src/traefik-pages/src
COPY Cargo.toml /usr/src/traefik-pages/Cargo.toml
COPY Cargo.lock /usr/src/traefik-pages/Cargo.lock

RUN cargo build --release

FROM debian:buster-slim

ENV PORT 5000

EXPOSE 5000

USER www-data

COPY --from=builder /usr/src/traefik-pages/target/release/traefik-pages /usr/local/bin/traefik-pages

# For reasons I don't know, the application throws away SIGTERM. There's explicit actix code to handle it, but it doesn't do anything.
# If you're reading this, and think you can help, please do!
# Requests will be fast enough that a hard terminate shouldn't be the end of the world.
STOPSIGNAL SIGKILL

ENTRYPOINT ["/usr/local/bin/traefik-pages"]
