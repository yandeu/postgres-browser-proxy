FROM alpine:3.16

RUN apk update
RUN apk upgrade

COPY ./target/x86_64-unknown-linux-musl/release/postgres-browser-proxy ./postgres-browser-proxy

CMD ["/postgres-browser-proxy"]