# First step we build the app
# Then we start from a new image to avoid tons of useless tools
FROM rust:1.45 as builder
WORKDIR /usr/src/backend
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/backend/target/release/rust-backend-example /usr/local/bin/backend

CMD ["backend"]
