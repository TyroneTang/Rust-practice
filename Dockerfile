FROM ubuntu:24.04 AS base

# ---------- ENV --------------- #

# Ensure apt uses the default answers
ENV DEBIAN_FRONTEND=noninteractive

ENV LANG=C.UTF-8
ENV LC_ALL=C.UTF-8
ENV HOME=/workspace

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



# ---------- Workspace --------------- #
# Non-root
# Rename pre-existing ubuntu user (UID 1000) to devuser, with /workspace as home
RUN usermod -l devuser ubuntu && \
    groupmod -n devuser ubuntu && \
    usermod -d /workspace devuser && \
    usermod -s /bin/bash devuser && \
    rm -rf /home/ubuntu

# Create workspace dirs and give ownership
RUN mkdir -p /workspace/fnm /workspace/project /workspace/.cache /workspace/.local /workspace/.config && \
    chown -R devuser:devuser /workspace

# Switch to non-root for everything else
USER root
RUN apt-get update && \
    apt-get install -y sudo && \
    echo "devuser ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers && \
    rm -rf /var/lib/apt/lists/*

# Pre-create rust install dirs so devuser can write into them
RUN mkdir -p /usr/local/rustup /usr/local/cargo && \
    chown -R devuser:devuser /usr/local/rustup /usr/local/cargo
USER devuser



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
