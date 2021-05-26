FROM rust:1.52.1

WORKDIR /usr/src/app

VOLUME /usr/local/cargo

ENV BACKEND_HOST=0.0.0.0:8081

EXPOSE 8081

COPY . .

CMD ["cargo","run"]