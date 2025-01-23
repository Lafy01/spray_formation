
# rust.dockerfile
# Build Stage
FROM rust:1.80 as builder

WORKDIR /app

ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY . .

RUN cargo build --release && echo "Build Successful" || echo "Build Failed"

RUN ls -la /app/target/release/

# Production Stage
FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/backend .

RUN chmod +x /app/backend

# EXPOSE 8080
CMD ["./backend"]