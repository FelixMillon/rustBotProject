
FROM rust:1.80

WORKDIR /usr/src/app

COPY . .

RUN cargo build

EXPOSE 3001

CMD ["cargo", "run"]