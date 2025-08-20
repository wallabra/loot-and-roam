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
RUN rustup-init -y --no-modify-path --default-toolchain nightly --component rustc-codegen-cranelift-preview \
    && echo "/root/.cargo/bin" >> /etc/profile.d/cargo.sh # Add cargo to PATH for future commands

ENV PATH="/root/.cargo/bin:${PATH}"

# TODO: consider pre-building dependencies
# - COPY Cargo.toml Cargo.lock ./
# - RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Clean up Alpine package cache
RUN rm -rf /var/cache/apk/*
