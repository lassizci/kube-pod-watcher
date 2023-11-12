FROM rust:1.73 as builder

WORKDIR /app

COPY src src
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/release/kube-pod-watcher /app

CMD ["/app/kube-pod-watcher"]
