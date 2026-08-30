# --- STAGE 1: Builder ---
FROM rustlang/rust:nightly-trixie AS builder

# Install build dependencies
RUN apt-get update -y && apt-get install -y --no-install-recommends \
    clang \
    libssl-dev \
    pkg-config \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-binstall & cargo-leptos
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y cargo-leptos

# 1. Set working directory first
WORKDIR /app/leptos-app

# 2. Copy source code (including rust-toolchain if present)
COPY . /app

# 3. Add WASM target to the ACTIVE toolchain resolved inside the working directory
RUN rustup target add wasm32-unknown-unknown

# 4. Build application
RUN cargo leptos build --release -vv

# --- STAGE 2: Runtime ---
FROM debian:trixie-slim AS runtime

WORKDIR /app

RUN apt-get update -y && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Note: Matching your binary name 'leptos-app' from the log output
COPY --from=builder /app/leptos-app/target/release/leptos-app /app/
COPY --from=builder /app/leptos-app/target/site /app/site
COPY --from=builder /app/leptos-app/Cargo.toml /app/

ENV RUST_LOG="info" \
    LEPTOS_SITE_ADDR="0.0.0.0:8080" \
    LEPTOS_SITE_ROOT="site"

EXPOSE 8080

CMD ["/app/leptos-app"]
