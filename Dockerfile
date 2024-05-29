FROM rust:1.78

WORKDIR /

RUN cargo install diesel_cli --no-default-features --features postgres &&\
    git clone https://github.com/iamhabi/server-news.git
    
WORKDIR /server-news/sql

ENTRYPOINT diesel setup &&\
    cd /server-news &&\
    cargo run --bin server
