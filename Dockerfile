FROM rust:1.90 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Use a minimal base image for the final artifact
FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=builder /app/target/release/gh-actions-sandbox /app/gh-actions-sandbox
CMD ["/app/gh-actions-sandbox"]
