lint:
    cargo clippy --all-features -- -D warnings

test:
    cargo test --all-features --no-fail-fast
    cargo test --no-default-features

fix:
    cargo fmt --all
    cargo clippy --all-features --fix --allow-dirty

# Everything you should do before opening a pull request.
pr: fix lint test

# No CPU assumptions
build:
    cargo build --all-features --release

build-local:
    RUSTFLAGS="-C target-cpu=native" cargo build --all-features --release

# Generic aarch64 target, no specific CPU. Assuming Neon support, as apparently
# most ARM CPUs support Neon instructions.
build-aarch64-cross:
    RUSTFLAGS="-C target-feature=+crt-static,+neon" cargo build --all-features --release --target=aarch64-unknown-linux-gnu

# I cannot get cross-compilation to work from a Pop!_OS 22.04 machine to Trixie-based
# Raspberry Pi OS. For cross-compilation (assuming model 4B), use
# "target-cpu=cortex-a72" and consider adding the +crt-static feature back in.
# Local Raspberry Pi 4B build.
build-rpi4b-local:
    RUSTFLAGS="-C target-cpu=native -C target-feature=+neon" cargo build --all-features --release

# NVIDIA Jetson. I believe all Jetsons have the same CPU, but verify
# via lscpu prior to building. Tested on Orin Nano.
build-jetson-cross:
    RUSTFLAGS="-C target-cpu=cortex-a78ae -C target-feature=+crt-static,+neon" cargo build --all-features --release --target=aarch64-unknown-linux-gnu

deb: build
    cargo deb --all-features --no-build

deb-local: build-local
    cargo deb --all-features --no-build

_deb-aarch64:
    cargo deb --target=aarch64-unknown-linux-gnu --no-build 

deb-aarch64-cross: build-aarch64-cross _deb-aarch64

deb-rpi4b-local: build-rpi4b-local
    cargo deb --no-build

deb-jetson-cross: build-jetson-cross _deb-aarch64

# Ensure you've first updated the version field in Cargo.toml.
# Requires GitHub CLI: https://github.com/cli/cli/blob/trunk/docs/install_linux.md
# Requires jq: apt install jq
# TODO: Use a changelog and put notes from changelog in release. For now, can manually
# edit it.
release: deb deb-aarch64-cross
    #!/usr/bin/env bash
    VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
    git tag -a "${VERSION}" -m "Release ${VERSION}"
    git push origin "${VERSION}"
    mkdir -p ./target/github
    cp ./target/release/disquip-bot ./target/github/disquip-bot-x86_64
    cp ./target/aarch64-unknown-linux-gnu/release/disquip-bot ./target/github/disquip-bot-aarch64
    gh release create "$VERSION" \
        --title "$VERSION" \
        --notes "$VERSION" \
        ./target/github/disquip-bot-x86_64 \
        ./target/github/disquip-bot-aarch64 \
        ./target/debian/*${VERSION}*.deb
