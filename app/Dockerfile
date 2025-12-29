FROM rust:latest

WORKDIR /usr/src/sc_manager_app

COPY . .

RUN rustup update stable

CMD ["cargo", "test", "--verbose"]