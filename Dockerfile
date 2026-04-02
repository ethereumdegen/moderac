FROM rust:1.85-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY sdk/ sdk/
COPY migrations/ migrations/

RUN cargo build --release --bin moderac-server

FROM node:22-bookworm-slim AS frontend

WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ .
RUN npm run build

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/moderac-server .
COPY --from=frontend /app/frontend/dist frontend/dist/

ENV LISTEN_ADDR=0.0.0.0:${PORT:-3000}

EXPOSE 3000

CMD ["./moderac-server"]
