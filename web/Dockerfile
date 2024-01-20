FROM rust:1.74-buster as builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .
RUN trunk build

FROM nginx:1.24-alpine
COPY --from=builder /usr/src/app/dist/* /usr/share/nginx/html/
