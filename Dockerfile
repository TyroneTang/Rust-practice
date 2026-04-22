FROM ubuntu:24.04 AS base

# ---------- ENV --------------- #

# Ensure apt uses the default answers
ENV DEBIAN_FRONTEND=noninteractive

ENV LANG=C.UTF-8
ENV LC_ALL=C.UTF-8

SHELL ["/bin/bash", "-c"]

# ---------- DEV UTILS --------------- #

RUN --mount=target=/var/lib/apt/lists/learn_rust,type=cache,sharing=locked \
    --mount=target=/var/cache/apt/learn_rust,type=cache,sharing=locked \
    apt update -y && \
    apt install -y \
    git \
    bash-completion \
    iproute2 \
    wget \
    nano \
    iputils-ping \
    openssh-client \
    curl \
    rsync \
    vim \
    build-essential \
    locales


# ---------- LANG PACK --------------- #

# RUN --mount=target=/var/lib/apt/lists/learn_rust,type=cache,sharing=locked \
#     --mount=target=/var/cache/apt/learn_rust,type=cache,sharing=locked \
#     apt update && \
#     apt install -y \
#     rustc
# RUN echo "rustc -V"

# ref: https://stackoverflow.com/a/57251636
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --no-modify-path --default-toolchain 1.95.0 \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME \
    && rustup --version && rustc --version

WORKDIR /workspace/project
