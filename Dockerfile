FROM mirror.gcr.io/rust:1.86.0-bullseye AS builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

ADD sudoku-cli sudoku-cli
ADD sudoku sudoku
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/sudoku-cli sudoku-cli

ENTRYPOINT ["/app/sudoku-cli"]
