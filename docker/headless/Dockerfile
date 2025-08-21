#------
# Development environment container
#------
FROM docker.io/frolvlad/alpine-glibc:latest
WORKDIR /app

# Build dependencies
RUN apk update && apk add --no-cache ca-certificates && apk add --no-cache \
    clang \
    lld \
    pkgconf \
    vulkan-loader-dev \
    alsa-lib-dev \
    git \
    openssl \
    openssh \
    nodejs \
    rustup

# Install Rustup, Rust's nightly toolchain, and the Cranelift codegen (good for Bevy development)
RUN rustup-init -y --no-modify-path --default-toolchain nightly --component rustc-codegen-cranelift-preview

# Add Rust toolchain to PATH
ENV PATH="${PATH}:/root/.cargo/bin"

ENV PATH="/root/.cargo/bin:${PATH}"

# Precompile dependencies (debug mode)
COPY Cargo.toml Cargo.lock ./

RUN \
    mkdir src && \
    echo 'fn main() {}' >src/main.rs && \
    echo '' >src/lib.rs && \
    cargo build --no-default-features

# Precompile dependencies (check mode)
RUN cargo check || true

# Cleanup
RUN rm -r src
RUN rm -rf /var/cache/apk/*
