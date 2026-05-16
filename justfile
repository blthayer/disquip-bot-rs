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
