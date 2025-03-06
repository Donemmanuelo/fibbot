FROM rust:latest As builder

WORkDIR /app

COPY . .

RUN cargo build --release 

RUN apt-get update && apt-get install libssl-dev

FROM debian:latest

COPY --from=builder /app/target/release/fib /app/fib

CMD ["/app/fib"]

ENTRYPOINT ["/app/fib"]
