FROM rust:alpine

COPY ./ /app

WORKDIR /app

RUN apk add --no-cache musl-dev
RUN cargo install systemfd cargo-watch -vvv
RUN cp -rvf /app/docker/dev/.env /app/.env
CMD systemfd --no-pid -s http::0.0.0.0:3000 -- cargo -vvv watch -x 'run --verbose'
