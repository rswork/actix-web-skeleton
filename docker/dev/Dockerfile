FROM rust

COPY ./ /app

WORKDIR /app

RUN cargo install systemfd cargo-watch -vvv
CMD systemfd --no-pid -s http::0.0.0.0:3000 -- cargo -vvv watch -x 'run --verbose'
