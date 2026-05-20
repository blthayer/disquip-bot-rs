# disquip-bot-rs

DisQuip Bot: Discord bot that plays audio clips from local files into voice channels
on command.

In essence, this is a customizable soundboard. Commands for randomizing aspects of
Civilization VI game setup are optionally included (assumes you have all the DLC).

https://github.com/user-attachments/assets/316cd5e1-277f-41c8-894f-0e7120be4d61

This is a re-implementation of the now defunct
[disquip-bot](https://github.com/blthayer/disquip-bot), originally written in
Python. Unfortunately the original DisQuip bot died an early death due to Discord
updating their API in a backwards-incompatible way.

## Table of Contents

1. [Disclaimers](#disclaimers)
1. [Quick Start](#quick-start)
1. [Usage Within Discord](#usage-within-discord)
1. [Setup, Install, and Run](#setup-install-and-run)

## Disclaimers

**This is a self-hosted bot** - *You* perform Discord configuration, collect your
own audio files, and run this program on your own PC, server, Raspberry Pi, etc.

**This is (working) beta software** - It is not guaranteed to work everywhere or be
100% secure, stable, or "productionized."

While everything **does** seem to work just fine in my environment (no obvious memory
leaks, no crashes after weeks of continuous runtime), testing is quite minimal, error
handling is minimal/incomplete, logging is mostly missing, not all edge cases are
covered, and security has not been assessed. Use at your own risk! No warranty is
implied or provided for this freely available software.

If you encounter any issues, please do file an issue or submit a pull request.

## Quick Start

1. Clone the repository and navigate to it:
   `git clone https://github.com/blthayer/disquip-bot-rs.git; cd disquip-bot-rs`.
1. Place the contents of your Discord bot's API token into a file called `token`
   in this directory.
1. Create subdirectories in the `audio` directory and populate them with `mp3`
   and/or `wav` files.
1. Compile and run locally: `./run.sh`

## Usage Within Discord

TL;DR: Type `!help` into a Discord server's text channel that the bot is authorized
to read from and write to, and go from there!

This section covers interacting with the bot/app through Discord, and assumes the
app is properly configured and the program is running. See [Quick Start](#quick-start)
or [Setup, Install, and Run](#setup-install-and-run) sections of this document for
more information on getting the bot running (launching the program).

All commands for the bot are prefixed with `!` and are entered into a text channel
that the bot is able to read and respond to messages in. In order to play audio
files, you must be in a voice channel.

This guide will not cover all commands in detail, as the `!help` contents should
stand on its own.

### Help Within Discord, Available Quip Categories

#### help

To get the available commands, type `!help`, which will give output similar to
the following:

```
Commands:
  !list                List quip categories or list quips for a given command. E.g., "!list" or "!list a1"
  !random              Aka "!r" or "!rand." Play a random quip.
  !disconnect          Disconnect the bot from its current voice channel.
  !dice                Roll the dice! Aka "!d." Usage: "!dice <n sides> <n dice>" - n dice defaults to 1
  !help                Show help menu.
  !civ_draft           Draw random leaders: "!civ_draft n_players n_leaders."
  !civ_list_modes      List game modes. Useful in conjunction with "!civ_draw_modes"
  !civ_draw_modes      Draw random game modes. See also "!civ_list_modes"
  !civ_draw_map        Draw a single random map.
  !civ_draw_settings   Draw random game settings to jump-start Civilization VI game setup.

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

The `!list` command lists available quip categories (which can then be used as
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
games in the `a1`-`a3` categories, clips from the Halo games in `halo`, Lord of the Rings
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

TL;DR: `!a3 2`

See [list](#list) first.

You must be in a voice channel for this to work. Simply type `!<category> <number>`
into the text channel you use for bot interactions, where `<category>` maps to a
directory of audio files, and the number is the counting number associated with the
file. See [Audio Files](#audio-files) for more information.

#### random

TL;DR: `!r`

Plays a globally random quip, or a random quip from a specified category
(`!r <category>`). This is a lot of fun and great for... discovering... quips
available to the bot.

## Setup, Install, and Run

**TL;DR**:

1. Set up Discord application, add to your server, download token, save to file
   with `600` permissions.
1. Create a directory containing subdirectories of `.mp3` and `.wav` audio files.
1. Run: `disquip-bot /path/to/audio /path/to/token`
1. Use: see [Usage Within Discord](#usage-within-discord)

**Installation methods**:

1. Download a pre-built binary from a [release](https://github.com/blthayer/disquip-bot-rs/releases)
1. Download and `apt install` a pre-packaged `.deb` from a
   [release](https://github.com/blthayer/disquip-bot-rs/releases)
1. Build binary or `.deb` from source yourself (recommend checking out a tag)

**Known working Linux systems**:

- Pop!_OS 22.04 LTS, x86_64 architecture - local build
- Raspberry Pi OS (Debian 13, a.k.a. "Trixie") April 2026 release, aarch64 architecture (Raspberry Pi 4 Model B Rev 1.5) - local build
- JetPack 6 (based on Ubuntu 22), aarch64 architecture (NVIDIA Jetson, Orin Nano) - cross-compilation

The bot very likely functions on other operating systems, but has not been tested on
any besides those listed here. Please submit a PR to add your setup and any
additional directions required. All directions here assume a Debian-based Linux
distribution (*e.g.* Ubuntu, Pop!_OS, etc.).

**NOTE**: Pre-built release binaries and recipes in the `justfile` use the
`--all-features` flag for `cargo`, meaning that the `civ` (Civilization VI)
feature is included. This feature does not add extra dependencies and is quite
lightweight. If you still wish to have a build without the `civ` features, build
from source yourself without the `--all-features` flag set (the `civ` features is
*not* enabled by default via `Cargo.toml`).

### Discord App Configuration

This assumes a [Discord App](https://docs.discord.com/developers/quick-start/overview-of-apps)
has already been created through Discord, the procedure for which is outside the
scope of this document.

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

DisQuip Bot is a "bring your own audio files" project - for legal and copyright
reasons, no audio files are distributed with the bot or this repository. Don't
let that discourage you - there are plenty of audio files
[available on the internet](https://aoe.heavengames.com/dl-php/showfile.php?fileid=1740).

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
  `Cargo.toml` and then run `cargo update symphonia`.
- After collecting all your audio files, consider normalizing them so the
  volume range is similar. The previous Python version of the bot leveraged
  [ffmpeg-normalize](https://github.com/slhck/ffmpeg-normalize) for this purpose.

### Prerequisites (Building From Source)

To build from source, the following tools are required:

- [Rust toolchain](https://rustup.rs/). Tested with the latest version
  (`rustc` version `1.95.0`), should work on older (but recent) versions as well.
- `gcc`: Easiest path is to `sudo apt update; sudo apt install build-essential`
- `cmake`: Simply `sudo apt update; sudo apt install cmake` on a Debian-based
  Linux system (*e.g.*, Debian, Ubuntu, Pop!_OS, Mint, *etc.*). Tested with
  versions `3.22.1` and `3.31.6`.

### Build from Source, Run Locally

For your convenience, simply run `./run.sh`. The script (and the program)
take two positional arguments: The path to your audio files and the path
to the file containing your Discord token. Example:

```bash
./run.sh audio token
```

### Pre-built Binaries

A limited set of pre-built binaries are provided for each
[release](https://github.com/blthayer/disquip-bot-rs/releases).

Run the binary with no arguments or `--help` to get help. Example:

Example:

```bash
./disquip-bot audio token
```

### crates.io

TODO

### `apt` / `systemd` (`.deb` files)

If you'd like to install `disquip-bot` with `apt` and run as a daemon (service)
managed by `systemd` with automatic program (re)start, follow the directions here.
This is especially useful if you have an always-on server like a Raspberry Pi.

Reasonable precautions have been taken to protect your system - namely, the service
is run as an unpriveleged dynamic user and the filesystem is mounted read-only. See
`systemd/disquip-bot.service` for more details.

A limited set of pre-built `.deb` packages are provided for each
[release](https://github.com/blthayer/disquip-bot-rs/releases). Download the `.deb`
approporate for your target machine.

If there is not an appropriate `.deb` for your OS/architecture, it's relatively easy
to create one for yourself. See the [Development](#development) section for
prerequisites to build from source. There are several `deb*` recipes in the `justfile`
- choose the one that best matches your use case. For building directly on the target,
use the `deb-local` recipe, *e.g.* `just deb-local`.

Prior to installation, we need to set up the Discord token and audio files. For the
token:

```bash
sudo mkdir /etc/disquip-bot
sudo touch /etc/disquip-bot/token
sudo chmod 600 /etc/disquip-bot/token
# Use your favorite editor to put the token in the file.
# Avoid printing the token to the shell console so it doesn't
# wind up in shell history.
sudo nano /etc/disquip-bot/token
```

The program will look for audio files in `/usr/share/disquip-bot/audio`. You can
directly place the audio file tree here (recommended), or use a symbolic link.
Direct placement:

```bash
sudo mkdir -p /usr/share/disquip-bot/audio
sudo cp -r /path/to/my/audio/* /usr/share/disquip-bot/audio/
```

Alternatively, using a symbolic link:

```bash
sudo mkdir /usr/share/disquip-bot
sudo ln -s /path/to/my/audio /usr/share/disquip-bot/audio
```

NOTE: permissions can be tricky with the symbolic link approach. The service is run
as a dynamic, unpriveleged user. This user must be able to read and execute (to list
files) in the audio directory, which will not work in your user's home directory,
except maybe in `~/Public`.

Finally, we're ready to install the program, which is as simple as:

```bash
sudo apt install ./name-of-deb.deb
```

where `name-of-deb.deb` is appropriately replaced with the file you either downloaded
or built locally.

#### Working with the Service

Check service health:

```bash
sudo systemctl status disquip-bot
```

Inspect the logs:

```bash
sudo journalctl -u disquip-bot
```

Prevent the service from running on boot:

```bash
sudo systemctl disable disquip-bot
```

Manually start the service:

```bash
sudo systemctl start disquip-bot
```

Manually stop the service:

```bash
sudo systemctl stop disquip-bot
```

Manually restart the service:

```bash
sudo systemctl restart disquip-bot
```

#### Uninstalling

```bash
sudo systemctl stop disquip-bot
sudo apt remove --purge disquip-bot
sudo rm -r /etc/disquip-bot/token
```

Additionally, consider deleting the audio files/directory at `/usr/share/disquip-bot`

#### Upgrading

Simply install a newer `.deb` via `apt`.

## Development

### Prerequisites

- [Rust toolchain](https://rustup.rs/)
- `cmake`: `sudo apt update; sudo apt install cmake`
- (Optional) [just](https://just.systems/man/en/installation.html): `cargo install --locked just`
- (Optional) [just-lsp](https://github.com/terror/just-lsp): `cargo install --locked just-lsp`
- (Optional) [cargo-deb](https://docs.rs/crate/cargo-deb/latest): `cargo install --locked cargo-deb`

If you choose not to install and use `just`, you can manually copy + run recipes from
the `justfile`, which will be referenced throughout.

### Cross-compiling (`aarch64-unknown-linux-gnu`)

Cross-compiling can be a bit of a headache. If possible, consider building directly
on your target instead. In the `justfile`, recipes that end in `-cross` signify
they are intended for cross-compilation, while those that end in `-local` signify
they should be run on the target.

```console
# One time installs:
sudo apt update
sudo apt install -y gcc-aarch64-linux-gnu
rustup target add aarch64-unknown-linux-gnu

# Build:
just build-aarch64-cross
# See also build-rpi4bi-local and build-jetson-cross
```
