FROM rust:1.45.1 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/moods-api-service
COPY . .

RUN apt-get update && apt-get -y install \
    openssl libssl-dev clang llvm-dev libclang-dev

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build  /usr/local/cargo/bin/moods /usr/src/moods-api-service

CMD ["moods"]
