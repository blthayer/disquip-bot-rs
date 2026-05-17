lint:
    cargo clippy --all-features -- -D warnings

test:
    cargo test --all-features --no-fail-fast
    cargo test --no-default-features

fix:
    cargo fmt --all
    cargo clippy --all-features --fix --allow-dirty

build:
    RUSTFLAGS="-C target-cpu=native" cargo build --all-features --release

# Generic aarch64 target, no specific CPU. Assuming Neon support, as apparently
# most ARM CPUs support Neon instructions.
build-aarch64:
    RUSTFLAGS="-C target-feature=+crt-static,+neon" cargo build --all-features --release --target=aarch64-unknown-linux-gnu

# For a generic aarch64 target, just remove the target-cpu.
# Tested on a Raspberry Pi Model 4B running TODO.
# For other RPi models, verify cpu with lscpu.
# TODO: Not working yet.
build-rpi4b:
    RUSTFLAGS="-C target-cpu=cortex-a72 -C target-feature=+crt-static,+neon" cargo build --all-features --release --target=aarch64-unknown-linux-gnu

# NVIDIA Jetson. I believe all Jetsons have the same CPU, but verify
# via lscpu prior to building. Tested on Orin Nano.
build-jetson:
    RUSTFLAGS="-C target-cpu=cortex-a78ae -C target-feature=+crt-static,+neon" cargo build --all-features --release --target=aarch64-unknown-linux-gnu

deb: build
    cargo deb --all-features --no-build

_deb-aarch64:
    cargo deb --target=aarch64-unknown-linux-gnu --no-build 

deb-aarch64: build-aarch64 _deb-aarch64

deb-rpi4b: build-rpi4b _deb-aarch64

deb-jetson: build-jetson _deb-aarch64

# Everything you should do before opening a pull request.
pr: fix lint test

release:
    echo "TODO: tag, build, upload"
