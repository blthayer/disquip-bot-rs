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

Ensure the bot is added/authenticated with the Discord server you intend to use (
directions outside the scope of this document).

## Development/Build dependencies

- `cmake`: Simply `sudo apt update; sudo apt install cmake` (older version
   seems to be okay)

## Run

First follow directions in the `Setup` section.

1. Build: `cargo build --release`
2. Run: `./run.sh`

## Interact

In a text channel, type "!hello" and send the message. The bot will respond with
"world."
