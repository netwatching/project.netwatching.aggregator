FROM ubuntu:24.04 AS base

WORKDIR /app

COPY /src/ /app/

ENTRYPOINT ["tail", "-f", "/dev/null"]

HEALTHCHECK --start-period=3s --interval=3s --timeout=3s CMD exit 0