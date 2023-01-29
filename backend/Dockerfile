FROM rust:latest as builder
WORKDIR /usr/src/backend

COPY . .
ENV SQLX_OFFLINE=true
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
EXPOSE 3000

COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend
CMD ["backend"]