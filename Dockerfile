FROM rust:latest as builder

WORkDIR /app

COPY . .

RUN cargo build --release 

FROM debian:latest

RUN apt-get update && apt-get install libssl-dev -y

COPY --from=builder /app/target/release/fib /app/fib

CMD ["/app/fib"]

ENTRYPOINT ["/app/fib"]
