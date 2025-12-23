FROM rust:1.90

WORKDIR /app

COPY bot/Cargo.toml bot/Cargo.lock ./

COPY bot/src ./src

RUN cargo build --release

CMD ["./target/release/bot"]