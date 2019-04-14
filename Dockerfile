FROM rust:1.31

WORKDIR /usr/src/consoletimer
COPY . .

RUN cargo install --path .

ENTRYPOINT ["consoletimer"]
