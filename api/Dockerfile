FROM golang:1.21-alpine as build

WORKDIR /app
COPY . .

RUN apk add make npm nodejs
RUN make build

FROM alpine:latest as run

WORKDIR /app

COPY --from=build /app/mbaraacom ./run
COPY --from=build /app/resources ./resources

EXPOSE 3000

CMD ["./run"]
