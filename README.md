# disquip-bot-rs

DisQuip Bot: Discord bot that plays audio clips from local files into voice channels
on command. Also includes commands for randomizing aspects of Civilization VI game
setup. Implemented in Rust.

This is a re-implementation of the now defunct
[disquip-bot](https://github.com/blthayer/disquip-bot), originally written in
Python. Unfortunately the original DisQuip but died an early death due to Discord
updating their API in a backwards-incompatible way.

## Quick Start

1. Clone the repository and navigate to it:
   `git clone https://github.com/blthayer/disquip-bot-rs.git; cd disquip-bot-rs`.
1. Place the contents of your Discord bot's API token into a file called `token`.
   in this directory.
1. Create subdirectories in the `audio` directory and populate them with `mp3` or
   `wav` files.
1. Compile and run: `./run.sh`

## Disclaimer

This software should be considered a beta. While everything seems to work just
fine and the bot is stable (no obvious memory leaks and no crahses after weeks of
continuous runtime), testing is quite minimal, error handling is minimal/incomplete,
and logging is missing. Use at your own risk!

If you encounter any issues, please do file an issue or submit a pull request.

## Usage

TL;DR: Type `!help` into a text channel and go from there!

This section covers interacting with the bot/app through Discord, and assumes the
app is properly configured and the program is running. See [Quick Start](#quick-start)
or [Setup, Install, and Run](#setup-install-and-run) sections of this document.

All commands for the bot are prefixed with `!` and go into a text channel the bot
is able to read and respond in. In order to play audio files, you must be in a
voice channel.

This guide will not cover all commands in detail, as the `!help` contents should
stand on its own.

### Help and Available Quip Categories

#### help

To get the available commands, type `!help`, which will give output similar to
the following:

```
Commands:
  !list             List quip categories or list quips for a given command. E.g., "!list" or "!list a1"
  !random           Aka "!r" or "!rand." Play a random quip.
  !disconnect       Disconnect the bot from its current voice channel.
  !civ_draft        Draw random leaders: "!civ_draft n_players n_leaders."
  !civ_list_modes   List game modes. Useful in conjunction with "!civ_draw_modes"
  !civ_draw_modes   Draw random game modes. See also "!civ_list_modes"
  !civ_draw_map     Draw a single random map.
  !help             Show help menu.

Type "!<category> <number>" (e.g., "a1 1") to play a quip!
Type "!list" to discover available quip categories.
Type "!list <category>" to get available quip numbers for the given category.
Type "!help <command>" for more info on a command.
```

It is then possible to get additional for commands via `!help <command>`. For
example, `!help list`:

```
!list

List quip categories or list quips for a given command. E.g., "!list" or "!list a1"

Parameters:
cat   (optional) 
```

#### list

The `!list` command list available quip categories (which can then be used as
commands), which are defined by the installed [audio files](#audio-files).
Example (truncated) output:

```
Quip categories:
a1
a2
a3
...
halo
lotr
misc
...
sw
```

For inspiration, my personal setup here includes the taunts from the Age of Empires
games in the a1-a3 categories, clips from the Halo games in `halo`, Lord of the Rings
movie audio clips in `lotr`, miscellaneous quips in `misc`, and of course clips from
Star Wars in `sw`.

To list out quips available for a given category, do `!list <category>`. For example,
in my setup `!list a3` yields the following taunts from Age of Empires 3:

```
1: "001 Yes.mp3"
2: "002 No.mp3"
3: "003 I Need Food.mp3"
4: "004 I Need Wood.mp3"
5: "005 I Need Coin.mp3"
...
```

To play the taunt that says "No," you would then type `!a3 2` into the text channel.

#### Playing a quip

TL;DR example: `!a3 2`

See [list](#list) first.

You must be in a voice channel for this to work. Simply type `!<category> <number>`
into the text channel you use for bot interactions, where `<category>` is maps to
a directory of audio files, and the number is the counting number associated with
the file. See [Audio Files](#audio-files) for more information.

#### random

TL;DR: `!r`

Plays a globally random quip, or a random quip from a specified category. This
is a lot of fun and great for... discovering... quips available to the bot.

## Setup, Install, and Run

This program is known to work on the following Linux systems:

- Pop!_OS 22.04 LTS, x86_64 architecture
- Debian 11 (bullseye), aarch64 architecture (Raspberry Pi 4 Model B Rev 1.5)

It very likely functions on other operating systems, but has not been tested on
any besides those listed here. Please submit a PR to add your setup and any
additional directions required.

### Prerequisites

- [Rust toolchain](https://rustup.rs/). Tested with `rustc` versions `1.92.0`
  and `1.93.1`.
- `cmake`: Simply `sudo apt update; sudo apt install cmake` on a Debian-based
  Linux system (*e.g.*, Debian, Ubuntu, Pop!_OS, Mint, *etc.*). Tested with
  version `3.22.1`.

### Discord App Configuration

This assumes a [Discord App](https://docs.discord.com/developers/quick-start/overview-of-apps)
has already been created through Discord. This procedure is outside the scope of
this document.

The following directions describe how to obtain an API token and how to configure
gateway intents.

1. Log into the Discord developer portal.
1. Under the "Bot" tab, click "Reset Token."
1. Copy the contents of the token into a file called `token` at the top-level of
   this repository (don't worry, it's ignored by `git`).
1. Back in the developer portal under the "Bot" tab, in the "Privileged Gateway
   Intents" sections toggle "Message Content Intent" on.
1. Save changes.

Ensure the bot is added/authenticated with the Discord server you intend to use (
directions outside the scope of this document).

### Audio Files

DisQuipt Bot is a "bring your own audio files" project - for legal and copyright
reasons, no audio files will be distributed with the bot.

1. Create subdirectories in the repository's top-level `audio` directory.
1. Populate the subdirectories with audio files (`.mp3` or `.wav`).

Tips:

- Do **NOT** use directory names that correspond to already built-in commands or
  their aliases. See the [help](#help) section of this document or use the `!help`
  command to get a listing of built-in commands and their aliases. As an obvious
  example, don't create a directory named `help`.
- Keep the directory names short as they'll be directly used as commands later.
  For instance, instead of a directory named `batman`, you may wish to name it
  `bm` for short.
- Use descriptive file names, as the file names are how users will discover
  quips. For instance, if one of your files contains the Governator saying
  "I'll be back," consider naming the file `I'll be back.mp3` (and maybe placing
  it in a directory called `tm`, short for Terminator).
- Keep the clips short! Your friends will be quite annoyed if you play clips
  that last more than a few seconds.
- For additional audio file format support, add to the `features` list of
  the [symphonia](https://docs.rs/crate/symphonia/latest) dependency in
  `Cargo.toml`.

### Run

For your convenience, simply run `./run.sh`.
