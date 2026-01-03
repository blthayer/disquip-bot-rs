#!/bin/bash

set -euo pipefail
cargo build --release
DISCORD_TOKEN="$(<token)"
export DISCORD_TOKEN
exec ./target/release/disquip-bot-rs audio
