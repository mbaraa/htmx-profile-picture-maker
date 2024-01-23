FROM golang:1.21-alpine as build

WORKDIR /app
COPY . .

RUN go build -ldflags="-w -s"

FROM alpine:latest as run

WORKDIR /app

COPY --from=build /app/hppm ./run
# COPY --from=build /app/resources ./resources

EXPOSE 8080

CMD ["./run"]
