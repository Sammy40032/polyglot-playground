FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo fetch && cargo build --release
CMD ["./target/release/polyglot-playground"]
