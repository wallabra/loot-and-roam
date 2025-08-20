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
RUN echo "source /root/.cargo/env" >> /etc/profile.d/cargo.sh

RUN cargo install cargo-build-deps # Used when precompiling deps

ENV PATH="/root/.cargo/bin:${PATH}"

# Precompile dependencies
COPY Cargo.toml Cargo.lock ./

RUN cargo build-deps --no-default-features

# Clean up Alpine package cache
RUN rm -rf /var/cache/apk/*
