FROM rust:1.74-buster as deps

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - &&\
    apt-get install -y nodejs
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

FROM rust:1.74-buster as builder
RUN apt install make

WORKDIR /usr/src/app
COPY . .

RUN make build

FROM alpine:latest
COPY --from=builder /usr/src/app/dist/ /opt/dist/
# COPY --from=builder /usr/src/app/dist/* /usr/share/nginx/html/
