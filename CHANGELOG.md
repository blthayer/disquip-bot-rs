# Changelog

Versions before `0.7.1` will not work with Discord due to the introduction of
end-to-end encryption via the DAVE protocol. The bot's feature set should
be relatively stable starting at version `0.9.1`.

For now, this CHANGELOG is hand-curated, so only versions starting at `0.9.0`
are documented here. Until a `1.0.0` release is published, minor patch bumps
may include breaking or backwards-incompatible changes.

## `0.9.2`

Update documentation, publish to crates.io.

## `0.9.1`

The bot now automatically leaves a voice channel (after a short delay) when there
are no longer any human users in the voice channel.

## `0.9.0`

Add three new commands:

- `search_exact`, *a.k.a.* `se`
- `search_fuzzy`, *a.k.a.* `sf`
- `lucky` (play by search), *a.k.a.* `l`
