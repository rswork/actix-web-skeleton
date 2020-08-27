# ------------------------------------------------------------------------------

# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/actix-web-skeleton*

COPY . .

RUN cp docker/prod/.env .env

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN mkdir /app

COPY --from=cargo-build /app/.env /app/.env

COPY --from=cargo-build /app/target/x86_64-unknown-linux-musl/release/actix-web-skeleton /usr/local/bin/binapp

WORKDIR /app

CMD binapp
