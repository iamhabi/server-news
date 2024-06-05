FROM rust:1.78

WORKDIR /mynews-server

RUN cargo install diesel_cli --no-default-features --features postgres
    
WORKDIR /mynews-server/sql

ENTRYPOINT diesel setup &&\
    cd .. &&\
    cargo run --bin server
