lint:
    cargo clippy --all-features -- -D warnings

test:
    cargo test --all-features --no-fail-fast

fix:
    cargo fmt --all
    cargo clippy --all-features --fix --allow-dirty
