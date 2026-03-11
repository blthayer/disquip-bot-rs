#!/bin/bash

set -euo pipefail
DISCORD_TOKEN="$(<token)"
export DISCORD_TOKEN
cargo run
