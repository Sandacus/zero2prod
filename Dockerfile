FROM lukemathwalker/cargo-chef:latest-rust-1.93.0 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
# create a pseudo lock file for the project
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build project dependencies, not the application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point our dependency tree stays the same
# all layers should be cached
COPY . .
ENV SQLX_OFFLINE=true
# Build our project
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /app
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	# Clean up
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the config file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT="production"
ENTRYPOINT ["./zero2prod"]
