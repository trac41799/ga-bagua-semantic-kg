FROM rust:1.78-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p ga-semantics-mcp

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ga-semantics-mcp /usr/local/bin/ga-semantics-mcp
EXPOSE 3100
ENV PORT=3100
CMD ["ga-semantics-mcp", "--http", "--port", "3100"]
