FROM rust:latest

RUN rustup default nightly

COPY Cargo.lock Cargo.toml ./
COPY src/ ./src/

RUN cargo build --release

ENV ROCKET_CLI_COLORS=off
ENV ROCKET_ENV=prod
EXPOSE 8000
CMD ["./target/release/rustshambo"]
