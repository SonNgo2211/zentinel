# Stage 1: Build
FROM rust:1.82-slim-bookworm AS builder

WORKDIR /usr/src/ai-auditor
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev cmake g++

COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/ai-auditor/target/release/zentinel-agent-ai-auditor /app/ai-auditor
COPY ./models /app/models

EXPOSE 50051

CMD ["./ai-auditor"]
