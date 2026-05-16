#!/bin/bash
# One argument: path to audio files directory (default: "audio").

set -euo pipefail
DISCORD_TOKEN="$(<token)"
export DISCORD_TOKEN
cargo run --release -- "${1:-audio}"
