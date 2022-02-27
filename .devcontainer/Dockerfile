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
    vim \
    curl

# install mold
ENV mold_version=v1.1
RUN git clone --branch "$mold_version" --depth 1 https://github.com/rui314/mold.git && \
    cd mold && \
    make -j$(nproc) CXX=clang++ && \
    make install && \
    mv /mold/mold /usr/bin/mold && \
    mv /mold/mold-wrapper.so /usr/bin/mold-wrapper.so && \
    make clean

# install gh and fish
RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
      | dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" \
      | tee /etc/apt/sources.list.d/github-cli.list > /dev/null && \
    echo 'deb http://download.opensuse.org/repositories/shells:/fish:/release:/3/Debian_11/ /' \
      | tee /etc/apt/sources.list.d/shells:fish:release:3.list && \
    curl -fsSL https://download.opensuse.org/repositories/shells:fish:release:3/Debian_11/Release.key \
      | gpg --dearmor \
      | tee /etc/apt/trusted.gpg.d/shells_fish_release_3.gpg > /dev/null && \
    apt-get update && \
    apt-get install -y \
      gh \
      fish && \
    rm -rf /var/lib/apt/lists/*

# use from rust-analyzer
RUN rustup component add rust-src && \
    rustup component add rust-analysis && \
    rustup component add rls && \
    rustup component add rustfmt

# vscode
RUN groupadd -g 1000 vscode && \
    useradd -m -s /bin/bash -u 1000 -g 1000 vscode
USER vscode
WORKDIR /vscode
RUN mkdir -p /vscode/target && chown vscode:vscode /vscode/target

# development utilities
RUN mold -run cargo install cargo-edit