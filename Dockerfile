# Use the official Rust image as the build environment
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Use a minimal base image for the runtime
FROM debian:bullseye-slim
WORKDIR /app
RUN apt-get update && apt-get upgrade -y && apt-get clean
COPY --from=builder /app/target/release/mudb /app/mudb
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
EXPOSE 6379
CMD ["/app/mudb"]
