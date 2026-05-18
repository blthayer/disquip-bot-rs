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

build-local:
    RUSTFLAGS="-C target-cpu=native" cargo build --all-features --release

# Generic aarch64 target, no specific CPU. Assuming Neon support, as apparently
# most ARM CPUs support Neon instructions.
build-aarch64-cross:
    RUSTFLAGS="-C target-feature=+crt-static,+neon" cargo build --all-features --release --target=aarch64-unknown-linux-gnu

# I cannot get cross-compilation to work from a Pop!_OS 22.04 machine to Trixie-based Raspberry Pi OS.
# For cross-compilation (assuming model 4B), use "target-cpu=cortex-a72" and consider
# adding the +crt-static feature back in.
# Local Raspberry Pi 4B build.
build-rpi4b-local:
    RUSTFLAGS="-C target-cpu=native -C target-feature=+neon" cargo build --all-features --release --target=aarch64-unknown-linux-gnu

# NVIDIA Jetson. I believe all Jetsons have the same CPU, but verify
# via lscpu prior to building. Tested on Orin Nano.
build-jetson-cross:
    RUSTFLAGS="-C target-cpu=cortex-a78ae -C target-feature=+crt-static,+neon" cargo build --all-features --release --target=aarch64-unknown-linux-gnu

deb-local: build-local
    cargo deb --all-features --no-build

_deb-aarch64:
    cargo deb --target=aarch64-unknown-linux-gnu --no-build 

deb-aarch64-cross: build-aarch64-cross _deb-aarch64

deb-rpi4b-local: build-rpi4b-local
    cargo deb --no-build

deb-jetson-cross: build-jetson-cross _deb-aarch64

release:
    echo "TODO: tag, build, upload"
