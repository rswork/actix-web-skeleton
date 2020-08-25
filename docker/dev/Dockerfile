FROM rust

COPY ./ /app

WORKDIR /app

RUN cargo install systemfd cargo-watch -vvv
CMD systemfd --no-pid -s http::3000 -- cargo -vvv watch -x run
