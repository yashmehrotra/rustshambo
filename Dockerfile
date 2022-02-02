FROM rust:latest

RUN rustup default nightly

COPY . .

RUN cargo build --release

ENV ROCKET_CLI_COLORS=off
ENV ROCKET_ENV=prod
CMD ["./target/release/rustshambo"]
