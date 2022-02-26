FROM rust:1.59.0-slim-bullseye

# install dependencies for build mold 
RUN apt-get update && \
    TZ=Asia/Tokyo apt-get install -y tzdata && \
    apt-get install -y \
    build-essential \
    git \
    clang \
    lld \
    cmake \
    libstdc++-10-dev \
    libssl-dev \
    libxxhash-dev \
    zlib1g-dev \
    pkg-config \
    vim && \
    rm -rf /var/lib/apt/lists/*

# install mold
ENV mold_version=v1.1
RUN git clone --branch "$mold_version" --depth 1 https://github.com/rui314/mold.git && \
    cd mold && \
    make -j$(nproc) CXX=clang++ && \
    make install && \
    mv /mold/mold /usr/bin/mold && \
    mv /mold/mold-wrapper.so /usr/bin/mold-wrapper.so && \
    make clean

# use from rust-analyzer
RUN rustup component add rust-src

# vscode
RUN groupadd -g 1000 vscode && \
    useradd -m -s /bin/bash -u 1000 -g 1000 vscode
USER vscode
WORKDIR /vscode
RUN mkdir -p /vscode/target && chown vscode:vscode /vscode/target

# development utilities
RUN mold -run cargo install cargo-edit cargo-watch