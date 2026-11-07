# syntax=docker/dockerfile:1
# The caller must select linux/amd64, including on Apple Silicon.
FROM ubuntu:22.04 AS build
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl wget build-essential pkg-config file xz-utils \
    libwebkit2gtk-4.1-dev libxdo-dev libssl-dev libayatana-appindicator3-dev \
    librsvg2-dev patchelf libfuse2 squashfs-tools xdg-utils \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY .nvmrc rust-toolchain.toml ./
RUN set -eu; version="$(cat .nvmrc)"; \
    curl -fsSLO "https://nodejs.org/dist/v${version}/node-v${version}-linux-x64.tar.xz"; \
    curl -fsSL "https://nodejs.org/dist/v${version}/SHASUMS256.txt" -o node-shasums.txt; \
    grep " node-v${version}-linux-x64.tar.xz$" node-shasums.txt | sha256sum --check -; \
    tar -xJf "node-v${version}-linux-x64.tar.xz" -C /usr/local --strip-components=1; \
    rm "node-v${version}-linux-x64.tar.xz" node-shasums.txt; \
    curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs -o /tmp/rustup.sh; \
    sh /tmp/rustup.sh -y --profile minimal --default-toolchain none; \
    rm /tmp/rustup.sh
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup show
COPY package.json package-lock.json .npmrc ./
RUN npm ci
COPY . .
ENV CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
ENV APPIMAGE_EXTRACT_AND_RUN=1
RUN npm run prepare:runtime
RUN npm run build && npm run check:rust && npm run test:rust
RUN npm run build:desktop:linux
RUN mkdir /out && cp src-tauri/target/release/bundle/appimage/*.AppImage /out/Coplayer-Linux-x86_64.AppImage \
    && cp scripts/linux/start-coplayer.sh /out/ \
    && cp docs/legion-proof.md /out/LEIA-ME.md \
    && chmod 755 /out/start-coplayer.sh /out/Coplayer-Linux-x86_64.AppImage \
    && cd /out && sha256sum Coplayer-Linux-x86_64.AppImage start-coplayer.sh LEIA-ME.md > SHA256SUMS
FROM scratch AS artifact
COPY --from=build /out/ /
