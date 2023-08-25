# Build stage
FROM rust:1.71-buster as builder

WORKDIR /app

# accept build args
ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

# Copy everything in the current folder to the docker image
COPY . .

RUN cargo build --release

# Production stage
FROM debian:buster-slim

# go to bin folder in debian docker image
WORKDIR /usr/local/bin

# remember our builder? we ran cargo build --release, so our code is there
COPY --from=builder /app/target/release/rust-crud-api .

# we already built, so let's just run!
CMD ["./rust-crud-api"]