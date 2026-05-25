#!/bin/bash
set -euo pipefail
cargo run --all-features -- "${1:-audio}" "${2:-token}"
