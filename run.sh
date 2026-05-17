#!/bin/bash
set -euo pipefail
RUSTFLAGS="-C target-cpu=native" cargo run --release --all-features -- "${1:-audio}" "${2:-token}"
