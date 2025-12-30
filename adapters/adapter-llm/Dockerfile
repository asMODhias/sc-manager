FROM rust:latest

WORKDIR /usr/src/adapter-llm

COPY . .

RUN rustup update stable

CMD ["cargo", "test", "--verbose"]
