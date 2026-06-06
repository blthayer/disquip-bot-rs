//! TL;DR:
//!
//! ```
//! cargo install --locked disquip-bot
//! disquip-bot /path/to/audio/dir /path/to/token/file
//! ```
//!
//! Alternatively, a limited set pre-built binaries (including `.deb` packages)
//! are available [on GitHub](https://github.com/blthayer/disquip-bot-rs/releases).
//!
//! This is a binary-only crate. `lib.rs` exists purely to support documentation
//! on docs.rs
//!
#![doc = include_str!("../README.md")]
#![doc = include_str!("../CHANGELOG.md")]
//!
//! # Feature flags
//! ## civ
//! Enables game setup and randomization commands for Civilization VI. Installed
//! by default in pre-built binaries (see GitHub releases) excluded by default when built from source via
//! `cargo`.
