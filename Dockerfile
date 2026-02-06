ARG PACKAGE
FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release -p ${PACKAGE}

FROM debian:bullseye-slim
ARG PACKAGE
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/${PACKAGE} /usr/local/bin/${PACKAGE}
WORKDIR /app
COPY assets /app/assets
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/${PACKAGE}"]
