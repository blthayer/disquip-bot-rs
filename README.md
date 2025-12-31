# disquip-bot-rs
Re-implementation of the now defunct [disquip-bot](https://github.com/blthayer/disquip-bot) in Rust

## Setup

This assumes an "App" in Discord parlance has already been created. Creating an
app/bot is outside the scope of this document.

1. Log into the Discord developer portal.
2. Under the "Bot" tab, click "Reset Token."
3. Copy the contents of the token into a file called `token`
4. Back in the developer portal under the "Bot" tab, in the "Privileged Gateway
   Intents" sections toggle "Message Content Intent" on.
5. Save changes.

## Usage

1. Build: `cargo build --release`
2. Run: `DISCORD_TOKEN="$(<token)" ./target/release/disquip-bot-rs`
