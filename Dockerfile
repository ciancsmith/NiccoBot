FROM rust:1.77.2 as builder
LABEL authors="Cian Crowley-Smith"

WORKDIR /niccobot

RUN apt-get update && apt-get install -y cmake pkg-config libopus-dev

# Copy the manifests
COPY ./Cargo.toml ./Cargo.lock ./

# If you have a workspace, copy the entire workspace directory
COPY . .

# Build the dependencies (and cache them)
RUN cargo build --release --bin niccobot
# Now remove the built binary to force a rebuild later
RUN rm -f target/release/deps/niccobot*

# Build the real application without touching the src/main.rs
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim
COPY --from=builder /niccobot/target/release/niccobot /usr/local/bin/niccobot
COPY --from=builder /niccobot/.env /niccobot/.env
COPY --from=builder /niccobot/migrations /niccobot/migrations
# Copy any other resources or files that the application requires at runtime
RUN apt-get update \
    && apt-get install -y libopus0 openssl bash coreutils \
    && rm -rf /var/lib/apt/lists/*
# Set environment variable to point to .env file
ENV DOTENV=/niccobot/.env

# Set the working directory
WORKDIR /niccobot

CMD ["niccobot"]