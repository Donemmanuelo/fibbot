FROM rust:latest As builder

WORkDIR /app

COPY . .  

RUN cargo build --release 

FROM debian:latest

RUN apt-get update && apt-get install -y libssl-dev 

COPY --from=builder /app/target/release/fib /app/fib

CMD ["/app/fib"]

ENTRYPOINT ["/app/fib"]
