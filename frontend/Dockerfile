FROM rust:1.52.1

RUN cargo install cargo-cmd
RUN cargo install miniserve
RUN cargo install wasm-pack

WORKDIR /usr/src/app

VOLUME /usr/local/cargo

ENV WASM_PROFILE=dev
ENV BACKEND_HOST=8080

EXPOSE 8080

COPY . .

CMD ["cargo","cmd","build-and-serve"]