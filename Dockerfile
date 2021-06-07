FROM rust:1.52 AS builder
WORKDIR /usr/src/consoletimer
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /root/
COPY --from=builder /usr/src/consoletimer/target/release/consoletimer .

ENTRYPOINT ["./consoletimer"]
