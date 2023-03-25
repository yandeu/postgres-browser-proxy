FROM alpine:3.16

RUN apk update
RUN apk upgrade
RUN apk add curl

COPY ./target/x86_64-unknown-linux-musl/release/postgres-browser-proxy ./postgres-browser-proxy

EXPOSE 3000

CMD ["/postgres-browser-proxy"]