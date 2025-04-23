FROM mirror.gcr.io/rust:1.86.0-bullseye AS builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

ADD benches benches
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD src src

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/sudoku sudoku

ENTRYPOINT ["/app/sudoku"]
