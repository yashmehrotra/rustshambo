FROM rust:latest as builder
RUN rustup default nightly
COPY Cargo.lock Cargo.toml ./
COPY src/ ./src/
COPY frontend ./frontend/
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM scratch
COPY --from=builder /target/x86_64-unknown-linux-gnu/release/rustshambo /
ENV ROCKET_CLI_COLORS=false
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["/rustshambo"]
