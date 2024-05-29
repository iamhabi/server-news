FROM rust:1.78

WORKDIR /

RUN cargo install diesel_cli --no-default-features --features postgres &&\
    git clone https://github.com/iamhabi/MyNews.git
    
WORKDIR /MyNews/sql

ENTRYPOINT diesel setup &&\
    cd /MyNews &&\
    cargo run --bin server
