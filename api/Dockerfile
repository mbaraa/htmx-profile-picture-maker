FROM golang:1.21-alpine as build

RUN apk add cairo cairo-dev go git
WORKDIR /app
COPY . .

RUN go build -ldflags="-w -s"

FROM alpine:latest as run
RUN apk add cairo
WORKDIR /app

COPY --from=build /app/hppm ./run
COPY --from=build /app/resources ./resources

EXPOSE 8080

CMD ["./run"]
