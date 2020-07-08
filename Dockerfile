FROM rust:1.43.1 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/actix-sample-api
COPY . .

RUN cargo install --path .

# as slim as alpine ( +- 5mb), but have all libs and we need to
# also it container created by Google, what why we can trust(+-) that container
FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/actix-sample-api /usr/local/bin/actix-sample-api

CMD ["actix-sample-api"]