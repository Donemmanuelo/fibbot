FROM rust:latest As builder

WORkDIR /app

COPY . .  

RUN cargo build --release 

FROM debian:latest

RUN apt-get update && apt-get install -y libssl-dev 

COPY --from=builder /app/target/release/fibb /app/fibb

CMD ["/app/fibb"]

ENTRYPOINT ["/app/fibb"]
