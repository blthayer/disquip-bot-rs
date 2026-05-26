#[cfg(feature = "civ")]
mod civ;
#[cfg(feature = "civ")]
use crate::civ::{GAME_MODES, draw_leaders, draw_map, draw_modes, draw_settings};
use ahash::AHashMap;
use poise::serenity_prelude as serenity;
use rand::{
    RngExt,
    distr::{Distribution, Uniform},
};
use songbird::SerenityInit;
use std::{
    env,
    fs::{DirEntry, read_dir},
};
// Event related imports to detect track creation failures.
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
type FileMap = AHashMap<String, Vec<DirEntry>>;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::PrefixContext<'a, Data, Error>;
type GenericContext<'a> = poise::Context<'a, Data, Error>;

struct Data {
    // Map of quip categories to directory entries.
    pub file_map: FileMap,
    // TODO: Could use &str here since it shares the same keys as
    // `file_map`, but I don't really feel like messing with lifetimes.
    // TODO: Could use a single map and store the transformed strings
    // in the file_map alongside the full DirEntry structs...
    /// Parallel to `file_map`, but stores lowercase file names only
    /// (no extra path info), sans extension.
    str_map: AHashMap<String, Vec<String>>,
    pub map_len: usize,
}

// TODO: Could use nicer error proliferation with anyhow instead of
// std::process:: sprinklings.
fn read_dir_or_exit(dir: &str) -> std::fs::ReadDir {
    match read_dir(dir) {
        Ok(rd) => rd,
        Err(e) => {
            eprintln!("Failed to read directory at {dir}: {e:?}");
            std::process::exit(1);
        }
    }
}

impl Data {
    fn new(top_dir: &str) -> Data {
        // Initialize the file map and a counter for the total number of DirEntries.
        let mut file_map: FileMap = AHashMap::new();
        let mut str_map: AHashMap<String, Vec<String>> = AHashMap::new();
        let mut map_len: usize = 0;

        // Loop over directories within the top_dir and fill out the AHashMap.
        let rd = read_dir_or_exit(top_dir);
        for r in rd {
            let u = r.unwrap();
            // Only work with directories.
            if u.file_type().unwrap().is_dir() {
                // Iterate over the files and place in the AHashMap using the
                // directory's name as a key.
                let key = u.file_name().into_string().unwrap();
                let files = read_dir_or_exit(u.path().to_str().unwrap());
                for f in files {
                    let dir_entry = f.unwrap();
                    let file_path = dir_entry.path();

                    // Insert into the file_map.
                    // TODO: This key cloning feels silly.
                    match file_map.entry(key.clone()) {
                        std::collections::hash_map::Entry::Occupied(mut oe) => {
                            oe.get_mut().push(dir_entry);
                        }
                        std::collections::hash_map::Entry::Vacant(ve) => {
                            ve.insert(vec![dir_entry]);
                        }
                    }

                    // Get the lowercase version of the file name, sans extension.
                    let str_entry = std::path::Path::new(&file_path)
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_lowercase();

                    // Insert into the str_map.
                    match str_map.entry(key.clone()) {
                        std::collections::hash_map::Entry::Occupied(mut oe) => {
                            oe.get_mut().push(str_entry);
                        }
                        std::collections::hash_map::Entry::Vacant(ve) => {
                            ve.insert(vec![str_entry]);
                        }
                    }
                    map_len += 1;
                }
            }
        }

        // Sort. In order to sort the file_map and the str_map consistently,
        // perform the same mutations for sorting (remove extension, lowercase).
        // Yes, it would probably be more efficient to get the sort order for
        // one and use it to sort the other...
        for val in file_map.values_mut() {
            val.sort_by_key(|k| {
                std::path::Path::new(&k.path())
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
            });
        }
        for val in str_map.values_mut() {
            val.sort();
        }
        Data {
            file_map,
            str_map,
            map_len,
        }
    }

    /// Get a `DirEntry` from the given index. The index is effectively an
    /// index into the imaginary vector of all `DirEntries` in the `FileMap`
    /// concatenated together. Also returns the chosen category and index within.
    fn get_from_global_index(&self, idx: usize) -> Result<(&DirEntry, String, usize), Error> {
        let mut visited: usize = 0;

        for (cat, vec) in &self.file_map {
            let this_idx = idx - visited;
            let len = vec.len();
            if (idx - visited) < len {
                return Ok((&vec[this_idx], cat.to_owned(), this_idx));
            }

            visited += len;
        }
        Err(format!("The provided idx ({}) to get_from_global_index is too large. Index must be between 0 and {}.", idx, self.map_len).into())
    }

    /// Get a vector from the `file_map` from the given key ("cat" for "category").
    /// If the key is not present, return an error which eventually gets floated up
    /// to the user.
    fn get_vec(&self, cat: &String) -> Result<&Vec<DirEntry>, Error> {
        if let Some(cat_vec) = self.file_map.get(cat) {
            return Ok(cat_vec);
        }

        Err(format!("The provided category {cat:?} is invalid. Use \"!list\" with no arguments to get valid categories.").into())
    }

    /// Compute Jaro-Winkler distance for all mapped files in the `file_map` via
    /// the `str_map` (case insensitive). `compare_to` is cast to lowercase prior
    /// to comparing with every string in the `str_map`. The returned vector is sorted
    /// by distance score (best to worst).
    fn ordered_distance(&self, compare_to: &str) -> Vec<(f64, String, usize)> {
        // Jaro is "often used in the field of record linkage and string matching"
        // and is "particularly effective in comparing short strings, such as names"
        //
        // Jaro-Winkler adds additional sensitivity to matching prefixes - probably
        // not ideal in this use case. For example, one might search "dad" to find
        // "You_re not my dad.mp3"
        let comparator =
            rapidfuzz::distance::jaro::BatchComparator::new(compare_to.to_lowercase().chars());

        let mut all_scores: Vec<(f64, String, usize)> = Vec::with_capacity(self.map_len);
        // This could of course be parallelized in the future. For now, KISS.
        for (cat, vec) in &self.str_map {
            for (idx, name) in vec.iter().enumerate() {
                let distance = comparator.distance(name.chars());
                all_scores.push((distance, cat.clone(), idx));
                // println!("{cat} {idx} {name} {distance:.2}");
            }
        }
        all_scores.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        all_scores
    }

    /// Check for exact matches via `contains` for every file name in the `str_map`.
    /// Case insensitive.
    fn name_contains(&self, contains: &str) -> Vec<(String, usize)> {
        let contains = contains.to_lowercase();
        let mut contains_vec: Vec<(String, usize)> = Vec::new();
        for (cat, vec) in &self.str_map {
            for (idx, name) in vec.iter().enumerate() {
                if name.contains(&contains) {
                    contains_vec.push((cat.clone(), idx));
                }
            }
        }
        contains_vec
    }
}

/// Play a quip!
#[poise::command(prefix_command, guild_only = true, hide_in_help = true)]
async fn join_and_play(ctx: Context<'_>, num: usize) -> Result<(), Error> {
    // Join the voice channel.
    join(&ctx).await?;

    // Get the chosen_file.
    let command = ctx.invoked_command_name().to_string();
    let file_vec = ctx.data().get_vec(&command)?;
    let attempt_chosen_file = file_vec.get(num - 1);
    let Some(chosen_file) = attempt_chosen_file else {
        ctx.say(
            format!(
                "The given integer \"{:?}\" is invalid. Valid integers for the {:?} command range from 1 to {:?}",
                num, command, file_vec.len()
            )
        ).await?;
        return Ok(());
    };
    play(&ctx, chosen_file).await?;
    Ok(())
}

async fn join(ctx: &Context<'_>) -> Result<(), Error> {
    // Get user's voice channel.
    let guild = ctx.guild().unwrap().to_owned();
    let user_id = ctx.author().id;
    let voice_states = guild.voice_states.get(&user_id);

    let Some(voice_states) = voice_states else {
        return Err("You must be in a voice channel to play quips!".into());
    };

    let Some(channel_id) = voice_states.channel_id else {
        return Err("Failed to get voice channel ID (which is very, very odd...)".into());
    };

    let songbird_id = songbird::id::ChannelId::from(channel_id);
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let guild_id = guild.id;
    // Exit early if we're already in the channel.
    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        if let Some(current_id) = handler.current_channel()
            && current_id == songbird_id
        {
            return Ok(());
        }
    }

    // It seems to be fine if there are multiple join calls, probably no need
    // to add our own conditional here.
    let handler_lock = manager.join(guild_id, songbird_id).await?;

    // Leave the channel when only bots remain.
    let cache = std::sync::Arc::clone(&ctx.serenity_context.cache);
    tokio::spawn(async move {
        loop {
            // Don't spam.
            tokio::time::sleep(tokio::time::Duration::from_mins(1)).await;
            // If there are not members in the channel, leave the channel.
            match guild.channels.get(&channel_id) {
                Some(guild_channel) => {
                    let members = guild_channel
                        .members(std::sync::Arc::clone(&cache))
                        .unwrap();
                    if members.iter().all(|m| m.user.bot) {
                        manager.remove(guild_id).await.unwrap();
                        return;
                    }
                }
                None => return,
            }
        }
    });

    let mut handler = handler_lock.lock().await;
    // Ensure there's only ever a single event/error handler:
    handler.remove_all_global_events();
    handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    Ok(())
}

struct TrackErrorNotifier;

#[serenity::async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}

async fn play(ctx: &Context<'_>, dir_entry: &DirEntry) -> Result<(), Error> {
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    // TODO: Double unwrapping here... Better error handling is more better.
    let handler_lock = manager.get(ctx.guild_id().unwrap()).unwrap();
    let mut handler = handler_lock.lock().await;

    let file = songbird::input::File::new(dir_entry.path());
    handler.play_only_input(file.into());

    Ok(())
}

/// Show help menu.
#[poise::command(prefix_command)]
pub async fn help(ctx: GenericContext<'_>, command: Option<String>) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
Type \"!<category> <number>\" (e.g., \"a1 1\") to play a quip!
Type \"!list\" to discover available quip categories.
Type \"!list <category>\" to get available quip numbers for the given category.
Type \"!help <command>\" for more info on a command.",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

/// List quip categories or list quips for a given command.
/// E.g., "!list" or "!list a1"
#[poise::command(prefix_command, guild_only = true)]
async fn list(ctx: Context<'_>, cat: Option<String>) -> Result<(), Error> {
    let data = ctx.data();
    let mut help_str = if let Some(cat_str) = cat {
        let cat_vec = data.get_vec(&cat_str)?;
        let mut help_str = format!("Available quips for category \"{cat_str}\":\n```\n");
        for (idx, item) in cat_vec.iter().enumerate() {
            help_str.push_str(
                format!(
                    "{}: {:?}\n",
                    u32::try_from(idx)? + 1,
                    item.file_name().into_string().unwrap()
                )
                .as_str(),
            );
        }
        help_str
    } else {
        let mut key_vec: Vec<String> = data.file_map.keys().cloned().collect();
        key_vec.sort();
        let mut help_str = String::from("Quip categories:\n```\n");
        for key in key_vec {
            help_str.push_str(format!("{key}\n").as_str());
        }
        help_str
    };
    if help_str.len() < 1997 {
        help_str.push_str("```");
        ctx.say(help_str).await?;
    } else {
        let to_say = split_str(&help_str);
        for say in to_say {
            ctx.say(say).await?;
        }
    }
    Ok(())
}

/// Split string to avoid Discord message limit. The string will be
/// surrounded with backticks for literal formatting.
fn split_str(to_split: &str) -> Vec<String> {
    // We could calculate the capacity, but that's overkill.
    let mut out = Vec::new();

    // I have no idea if this code is "good," but it works?
    // My understanding of strings is apparently not adequately
    // deep. There's probably a way to do this without copying,
    // but oh well.
    for (idx, chunk) in to_split
        .chars()
        .collect::<Vec<_>>()
        .chunks(1994)
        .enumerate()
    {
        let mut to_push = if idx == 0 {
            String::new()
        } else {
            String::from("```")
        };
        let chunk_str: String = chunk.iter().collect();
        to_push.push_str(chunk_str.as_str());
        to_push.push_str("```");
        out.push(to_push);
    }
    out
}

/// Disconnect the bot from its current voice channel.
#[poise::command(prefix_command, guild_only = true)]
async fn disconnect(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().to_owned();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    manager.remove(guild.id).await?;
    Ok(())
}

/// Aka "!r" or "!rand." Play a random quip.
///
/// E.g., `!r` to play a globally random quip or `!r a1` to play a random
/// quip from the "a1" category.
#[poise::command(prefix_command, guild_only = true, aliases("r", "rand"))]
async fn random(ctx: Context<'_>, cat: Option<String>) -> Result<(), Error> {
    // Join the voice channel.
    join(&ctx).await?;

    let data = ctx.data();
    // Use a block here because the rng needs dropped before the await later.
    let chosen_file: &DirEntry;
    let chosen_category: String;
    let idx: usize;
    {
        // Docs say this is a fast, pre-initialized generator. So it should
        // be cheap to get it, and it's probably not worth fighting through
        // the thread safety stuff to put the rng on the Data struct as a field.
        let mut rng = rand::rng();
        if let Some(cat_str) = cat {
            let file_vec = data.get_vec(&cat_str)?;
            idx = rng.random_range(0..file_vec.len());
            chosen_file = &file_vec[idx];
            chosen_category = cat_str;
        } else {
            (chosen_file, chosen_category, idx) =
                data.get_from_global_index(rng.random_range(0..data.map_len))?;
        }
    };
    ctx.say(format!(
        "```Playing quip \"{} {}\" ({})```",
        chosen_category,
        // Convert to 1-based indexing.
        idx + 1,
        chosen_file.file_name().into_string().unwrap()
    ))
    .await?;
    play(&ctx, chosen_file).await?;
    Ok(())
}

/// Fuzzy search for quips by file name. Aka "!sf." E.g., "!sf foo".
///
/// Optionally include a maximum number of results (10 by default),
/// *e.g.* `!search_fuzzy foo 20`.
///
/// Wrap multiple words in quotes, *e.g.* `!sf "foo bar"`
///
/// Use fuzzy search when you have an inexact search term, otherwise use
/// exact search (see `search_exact`).
///
/// The search is case insensitive, fuzzy, does not include the category
/// (only the file name), and excludes the file extension. The fuzzy search
/// method used is [Jaro similarity](https://docs.rs/rapidfuzz/latest/rapidfuzz/distance/jaro/index.html).
#[poise::command(prefix_command, aliases("sf",))]
async fn search_fuzzy(ctx: Context<'_>, search_for: String, n: Option<usize>) -> Result<(), Error> {
    let n = match n {
        None => 10,
        Some(n) => {
            if n > 100 {
                ctx.say("\"n\" must be less than or equal to 100. Try again.")
                    .await?;
                return Ok(());
            }
            n
        }
    };
    let data = ctx.data();
    // This will never be empty since we're scoring every single file and
    // returning them all.
    let scores = data.ordered_distance(&search_for);
    let score_slice = &scores[0..std::cmp::min(n, scores.len())];
    let mut to_say = String::from("```");
    for (_, cat, idx) in score_slice {
        let name = &data.file_map.get(cat).unwrap()[*idx]
            .file_name()
            .into_string()
            .unwrap();
        to_say.push_str(format!("!{cat} {}: \"{name}\"\n", idx + 1).as_str());
    }

    if to_say.len() < 1997 {
        to_say.push_str("```");
        ctx.say(to_say).await?;
    } else {
        let to_say = split_str(&to_say);
        for say in to_say {
            ctx.say(say).await?;
        }
    }
    Ok(())
}

/// Exact search for quips by file name. Aka "!se." E.g., "!se foo".
///
/// Optionally include a maximum number of results (10 by default),
/// *e.g.* `!search_exact foo 20`.
///
/// Wrap multiple words in quotes, *e.g.* `!se "foo bar"`
///
/// Alternatively use fuzzy search when you have an inexact search term. See
/// `search_fuzzy`.
///
/// The search is case insensitive, does not include the category (only the
/// file name), and excludes the file extension. This search is effectively
/// checking if `search_for` is exactly contained in a quip file name.
#[poise::command(prefix_command, aliases("se",))]
async fn search_exact(ctx: Context<'_>, search_for: String, n: Option<usize>) -> Result<(), Error> {
    let n = match n {
        None => 10,
        Some(n) => {
            if n > 100 {
                ctx.say("\"n\" must be less than or equal to 100. Try again.")
                    .await?;
                return Ok(());
            }
            n
        }
    };
    let data = ctx.data();
    let matches = data.name_contains(&search_for);
    if matches.is_empty() {
        ctx.say("No matches.").await?;
        return Ok(());
    }
    let matches_slice = &matches[0..std::cmp::min(n, matches.len())];
    let mut to_say = String::from("```");
    for (cat, idx) in matches_slice {
        let name = &data.file_map.get(cat).unwrap()[*idx]
            .file_name()
            .into_string()
            .unwrap();
        to_say.push_str(format!("!{cat} {}: \"{name}\"\n", idx + 1).as_str());
    }

    if to_say.len() < 1997 {
        to_say.push_str("```");
        ctx.say(to_say).await?;
    } else {
        let to_say = split_str(&to_say);
        for say in to_say {
            ctx.say(say).await?;
        }
    }
    Ok(())
}

/// Play by search. Aka "!l." E.g., "!l foo". Uses fuzzy search by default.
///
/// Wrap multiple words in quotes, *e.g.* `!l "foo bar"`.
///
/// To use exact search, add a trailing "e", *e.g.* `!l "foo bar" e`
/// or `!l foo e`.
///
/// For fuzzy search, the quip whose name is the best match is played. For
/// exact search, the first matching quip is played (which is somewhat random).
///
/// See `search_fuzzy` and `search_exact` for details on the search aspect.
#[poise::command(prefix_command, aliases("l",))]
async fn lucky(ctx: Context<'_>, search_for: String, e: Option<String>) -> Result<(), Error> {
    let data = ctx.data();
    let to_get: (String, usize) = match e {
        None => {
            // Will never be empty.
            let mut scores = data.ordered_distance(&search_for);
            let first = std::mem::take(&mut scores[0]);
            (first.1, first.2)
        }
        Some(user_str) => {
            if user_str != "e" {
                ctx.say(format!("Unexpected argument: {user_str}")).await?;
                return Ok(());
            }

            let mut results = data.name_contains(&search_for);

            if results.is_empty() {
                ctx.say("No matches.").await?;
                return Ok(());
            }

            std::mem::take(&mut results[0])
        }
    };

    let to_play = &data.file_map.get(&to_get.0).unwrap()[to_get.1];

    // Join the voice channel.
    join(&ctx).await?;
    ctx.say(format!(
        "```Playing quip \"{} {}\" ({})```",
        to_get.0,
        // Convert to 1-based indexing.
        to_get.1 + 1,
        to_play.file_name().into_string().unwrap()
    ))
    .await?;
    play(&ctx, to_play).await?;

    Ok(())
}

#[cfg(feature = "civ")]
#[allow(clippy::doc_markdown)]
/// Draw random leaders: "!civ_draft n_players n_leaders."
///
/// Example: `!civ_draft 4 5` to draw five leaders each for four players.
///
/// There will be no duplicate leaders or civilizations.
#[poise::command(prefix_command)]
async fn civ_draft(ctx: Context<'_>, n_players: usize, n_leaders: usize) -> Result<(), Error> {
    // Draw leaders.
    let leaders = draw_leaders(n_players * n_leaders);

    let mut leader_str = String::from("```\n");

    for (idx, slice) in leaders.chunks(n_leaders).enumerate() {
        leader_str.push_str(format!("Player {}:\n", idx + 1).as_str());
        let mut sub_str = String::new();
        for leader in slice {
            sub_str.push_str(format!("    {}: {}\n", leader.name, leader.civ).as_str());
        }
        // Hacky, but it works. Remove the last newline.
        sub_str.pop();
        leader_str.push_str(sub_str.as_str());
        leader_str.push('\n');
    }

    let to_say = split_str(&leader_str);
    for say in to_say {
        // tickified handled outside of this loop
        ctx.say(say).await?;
    }
    Ok(())
}

#[cfg(feature = "civ")]
#[allow(clippy::doc_markdown)]
/// List game modes. Useful in conjunction with "!civ_draw_modes"
#[poise::command(prefix_command)]
async fn civ_list_modes(ctx: Context<'_>) -> Result<(), Error> {
    let mut to_say = String::new();
    for (idx, mode) in GAME_MODES.iter().enumerate() {
        to_say.push_str(format!("{}: {}\n", idx + 1, mode).as_str());
    }
    to_say.pop();
    ctx.say(to_say).await?;
    Ok(())
}

#[cfg(feature = "civ")]
#[allow(clippy::doc_markdown)]
/// Draw random game modes. See also "!civ_list_modes"
///
/// Examples:
///
///     Draw a random number of modes: `!civ_draw_modes`
///     Draw 3 random modes: `!civ_draw_modes 3`
///     Draw a random number of modes, but exclude "Apocalypse" and "Dramatic Ages" modes: `!civ_draw_modes 0 1 3`
///     Draw 2 modes, exluding "Zombie Defense": `!civ_draw_modes 2 8`
///
/// Parameters:
///
///     n: Number of modes to draw. Must be set if using "exclude." Set to 0 (or don't set) for a random number of modes.
///     exclude: Space separated integers for modes to include. Use `!civ_list_modes` to get the mapping of integers to modes.
#[poise::command(prefix_command)]
async fn civ_draw_modes(
    ctx: Context<'_>,
    n: Option<usize>,
    exclude: Vec<usize>,
) -> Result<(), Error> {
    // Validate n.
    let n = match n {
        Some(n_usize) => {
            let n_modes = GAME_MODES.len();
            if n_usize > n_modes {
                ctx.say(format!(
                    "You provided n={n_usize}, but n must be <= {n_modes}."
                ))
                .await?;
                return Ok(());
            }

            if n_usize == 0 { None } else { n }
        }
        None => None,
    };

    // Validate exclude.
    let exclude = if exclude.is_empty() {
        None
    } else {
        if !exclude.iter().all(|x| (1..=GAME_MODES.len()).contains(x)) {
            ctx.say(format!(
                "For \"exclude,\" all values must be between 1 and {}, inclusive. You gave {:?}.",
                GAME_MODES.len(),
                exclude
            ))
            .await?;
            return Ok(());
        }
        Some(exclude.as_slice())
    };

    let modes = draw_modes(n, exclude);

    if modes.is_empty() {
        ctx.say("RNJesus says there will be no extra game modes (drew 0 when randomly determining the number of modes).").await?;
        return Ok(());
    }

    let mut to_say = String::new();
    for mode in modes {
        to_say.push_str(format!("{mode}\n").as_str());
    }
    to_say.pop();
    ctx.say(tickify(&to_say)).await?;
    Ok(())
}

#[cfg(feature = "civ")]
/// Draw a single random map.
#[poise::command(prefix_command)]
async fn civ_draw_map(ctx: Context<'_>) -> Result<(), Error> {
    let map = draw_map();
    ctx.say(tickify(map)).await?;
    Ok(())
}

#[cfg(feature = "civ")]
/// Draw random game settings to jump-start Civilization VI game setup.
#[poise::command(prefix_command)]
async fn civ_draw_settings(ctx: Context<'_>) -> Result<(), Error> {
    // Already tickified
    ctx.say(draw_settings()).await?;
    Ok(())
}

/// Roll the dice! Aka "!d." Usage: "!dice <n sides> <n dice>" - n dice defaults to 1
///
/// Examples:
///   - `!d 6 2` to roll two six-sided dice.
///   - `!d 20` to roll a single twenty-sided die.
///   - `!d 2` to emulate a coin flip (1 -> heads, 2 -> tails).
#[poise::command(prefix_command, aliases("d"))]
async fn dice(ctx: Context<'_>, n_sides: u32, n_dice: Option<usize>) -> Result<(), Error> {
    let n_dice = n_dice.unwrap_or(1);

    // Prevent someone from entering a stupidly large number. Hopefully whatever parsing
    // is done behind the scenes will handle bad input for n_sides, e.g. bigger than the
    // max u32.
    if n_dice > 100 {
        ctx.say("n_dice must be less than or equal to 100.").await?;
        return Ok(());
    }

    // Minimum would be a coin flip, aka two sides.
    if n_sides < 2 {
        ctx.say("n_sides must be greater than or equal to 2.")
            .await?;
        return Ok(());
    }

    // Get to work.
    let mut results: Vec<u32> = Vec::with_capacity(n_dice);
    // Scope this so the ThreadRng is dropped before the await point.
    {
        let mut rng = rand::rng();
        // It's okay to unwrap this since we've validated input.
        let uniform = Uniform::new_inclusive(1, n_sides).unwrap();

        for _ in 0..n_dice {
            results.push(uniform.sample(&mut rng));
        }
    }
    let to_say = results
        .iter()
        .map(|val| format!("{val}"))
        .collect::<Vec<String>>()
        .join(", ");
    ctx.say(tickify(&to_say)).await?;
    Ok(())
}

fn tickify(text: &str) -> String {
    format!("```{text}```")
}
// Exits the process.
fn usage() -> ! {
    println!(
"Usage: disquip-bot-rs [-h | --help] /path/to/audio/files /path/to/token

E.g.: \"disquip-bot-rs audio token\" for an \"audio\" directory and \"token\" file in the current directory.

It is recommended that the token file use 600 permissions for security. Avoid leaking credentials to shell history when creating the file.

Currently, \"mp3\" and \"wav\" audio files are supported. Additional formats can be enabled at compile-time by adding features to the \"symphonia\" dependency in the project's \"Cargo.toml\" file. This requires building from source. 
"
    );
    std::process::exit(1);
}

fn parse_args() -> (String, String) {
    let mut args: Vec<String> = env::args().collect();

    if args.iter().any(|e| {
        let trimmed = e.trim();
        trimmed == "--help" || trimmed == "-h"
    }) {
        usage();
    }

    let n: usize = 3;
    let (top_dir, token_path) = match args.len().cmp(&n) {
        std::cmp::Ordering::Less | std::cmp::Ordering::Greater => {
            eprintln!(
                "Received {} arguments, expected {}\n",
                args.len() - 1,
                n - 1
            );
            usage();
        }
        std::cmp::Ordering::Equal => (std::mem::take(&mut args[1]), std::mem::take(&mut args[2])),
    };

    (top_dir, token_path)
}

#[tokio::main]
async fn main() {
    // Collect arguments
    let (top_dir, token_path) = parse_args();

    // Read the token from file.
    let token = match std::fs::read_to_string(&token_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading token file at {token_path}: {e:?}");
            std::process::exit(1);
        }
    };
    println!("Discord token successfully read from \"{token_path}.\"");

    let data = Data::new(&top_dir);
    println!("One time mapping of audio directory \"{top_dir}\" completed.");

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_VOICE_STATES;

    let prefix_framework_options = poise::PrefixFrameworkOptions {
        prefix: Some("!".to_string()),
        ..Default::default()
    };

    let mut command = join_and_play();
    let mut keys: Vec<_> = data.file_map.keys().cloned().collect();
    keys.sort();
    command.aliases = data.file_map.keys().cloned().collect();

    #[cfg(not(feature = "civ"))]
    let commands = vec![
        list(),
        search_exact(),
        search_fuzzy(),
        lucky(),
        random(),
        disconnect(),
        dice(),
        help(),
        command,
    ];

    #[cfg(feature = "civ")]
    let commands = vec![
        list(),
        search_exact(),
        search_fuzzy(),
        lucky(),
        random(),
        disconnect(),
        dice(),
        help(),
        civ_draft(),
        civ_list_modes(),
        civ_draw_modes(),
        civ_draw_map(),
        civ_draw_settings(),
        command,
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: prefix_framework_options,
            commands,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let mut client = match serenity::ClientBuilder::new(token.trim(), intents)
        .framework(framework)
        .register_songbird()
        .await
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error starting serenity client: {e:?}");
            std::process::exit(1);
        }
    };
    println!("Serenity Discord client initialized, about to start it and run forever...");
    if let Err(e) = client.start().await {
        eprintln!("The serenity client has failed: {e:?}");
        std::process::exit(1);
    }
}
