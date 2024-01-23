FROM rust:1.74-buster as builder

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - &&\
    apt-get install -y nodejs
RUN apt install make
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

WORKDIR /usr/src/app
COPY . .

# RUN cargo install --path .
# RUN trunk build
RUN make build

FROM alpine:latest
COPY --from=builder /usr/src/app/dist/ /opt/dist/
# COPY --from=builder /usr/src/app/dist/* /usr/share/nginx/html/
