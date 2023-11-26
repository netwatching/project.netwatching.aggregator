FROM rust:1.74-alpine as builder
WORKDIR /usr/src/myapp

RUN apk add --no-cache musl-dev binutils
COPY . .
RUN cargo install --path .
RUN strip /usr/local/cargo/bin/netwatching-aggregator

FROM alpine

COPY --from=builder /usr/local/cargo/bin/netwatching-aggregator /usr/local/bin/netwatching-aggregator
CMD ["netwatching-aggregator"]

HEALTHCHECK --start-period=3s --interval=3s --timeout=3s CMD exit 0