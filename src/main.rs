use poise::serenity_prelude as serenity;
use rand::Rng;
use songbird::SerenityInit;
use std::{
    collections::HashMap,
    env,
    fs::{DirEntry, read_dir},
};
// Event related imports to detect track creation failures.
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
type FileMap = HashMap<String, Vec<DirEntry>>;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::PrefixContext<'a, Data, Error>;
type GenericContext<'a> = poise::Context<'a, Data, Error>;

struct Data {
    // Map of quip categories to directory entries.
    pub file_map: FileMap,
    pub map_len: usize,
}

impl Data {
    fn new(top_dir: String) -> Data {
        // Initialize the file map and a counter for the total number of DirEntries.
        let mut file_map: HashMap<String, Vec<DirEntry>> = HashMap::new();
        let mut map_len: usize = 0;

        // Loop over directories within the top_dir and fill out the HashMap.
        let result = read_dir(top_dir).unwrap();
        for r in result {
            let u = r.unwrap();
            // Only work with directories.
            if u.file_type().unwrap().is_dir() {
                // Iterate over the files and place in the HashMap using the
                // directory's name as a key.
                let key = u.file_name().into_string().unwrap();
                let files = read_dir(u.path()).unwrap();
                for f in files {
                    let _f = f.unwrap();
                    match file_map.entry(key.to_owned()) {
                        std::collections::hash_map::Entry::Occupied(mut oe) => {
                            oe.get_mut().push(_f);
                        }
                        std::collections::hash_map::Entry::Vacant(ve) => {
                            ve.insert(vec![_f]);
                        }
                    }
                    map_len += 1;
                }
            }
        }

        // Sort.
        for val in file_map.values_mut() {
            val.sort_by_key(|a| a.path());
        }
        Data { file_map, map_len }
    }

    /// Get a DirEntry from the given index. The index is effectively an
    /// index into the imaginary vector of all DirEntries in the FileMap
    /// concatenated together. Also returns the chosen category and index within.
    fn get_from_global_index(&self, idx: usize) -> Result<(&DirEntry, String, usize), Error> {
        let mut visited: usize = 0;

        for (cat, vec) in self.file_map.iter() {
            let _idx = idx - visited;
            let _len = vec.len();
            if (idx - visited) < _len {
                return Ok((&vec[_idx], cat.to_owned(), _idx));
            };

            visited += _len;
        }
        Err(format!("The provided idx ({}) to get_from_global_index is too large. Index must be between 0 and {}.", idx, self.map_len).into())
    }

    /// Get a vector from the file_map from the given key ("cat" for "category").
    /// If the key is not present, return an error which eventually gets floated up
    /// to the user.
    fn get_vec(&self, cat: &String) -> Result<&Vec<DirEntry>, Error> {
        if let Some(cat_vec) = self.file_map.get(cat) {
            return Ok(cat_vec);
        };

        // TODO: How does "into" work?
        Err(format!("The provided category {:?} is invalid. Use \"!list\" with no arguments to get valid categories.", cat).into())
    }
}

/// Play a quip!
#[poise::command(prefix_command, guild_only = true, hide_in_help = true)]
async fn join_and_play(
    ctx: Context<'_>,
    #[description = "Quip number"] num: usize,
) -> Result<(), Error> {
    // Join the voice channel.
    join(&ctx).await?;

    // Get the chosen_file.
    let command = ctx.invoked_command_name().to_string();
    let file_vec = ctx.data().get_vec(&command)?;
    let attempt_chosen_file = file_vec.get(num - 1);
    let chosen_file = match attempt_chosen_file {
        Some(chosen_file) => chosen_file,
        None => {
            ctx.say(format!("The given integer \"{:?}\" is invalid. Valid integers for the {:?} command range from 1 to {:?}", num, command, file_vec.len()))
            .await?;
            return Ok(());
        }
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

    // Exit early if we're already in the channel.
    if let Some(handler_lock) = manager.get(ctx.guild_id().unwrap()) {
        let handler = handler_lock.lock().await;

        if let Some(current_id) = handler.current_channel()
            && current_id == songbird_id
        {
            return Ok(());
        }
    };

    // It seems to be fine if there are multiple join calls, probably no need
    // to add our own conditional here.
    let handler_lock = manager.join(guild.id, songbird_id).await?;
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
pub async fn help(
    ctx: GenericContext<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
Type \"!category number\" (e.g., \"aoe1 1\") to play a quip!
Type \"!list\" to discover available quip categories.
Type \"!list category\" to get available quip numbers for the given category.
Type \"!help command\" for more info on a command.",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

/// List available quip categories or list available quips for a given command.
#[poise::command(prefix_command, guild_only = true)]
async fn list(
    ctx: Context<'_>,
    #[description = "Quip category"] cat: Option<String>,
) -> Result<(), Error> {
    let data = ctx.data();
    match cat {
        Some(_cat) => {
            let cat_vec = data.get_vec(&_cat)?;
            let mut help_str = format!("Available quips for category \"{}\":\n```\n", _cat);
            for (idx, item) in cat_vec.iter().enumerate() {
                help_str.push_str(
                    format!(
                        "{}: {:?}\n",
                        idx as u32 + 1,
                        item.file_name().into_string().unwrap()
                    )
                    .as_str(),
                );
            }
            if help_str.len() < 1996 {
                help_str.push_str("\n```");
                ctx.say(help_str).await?;
            } else {
                // Fix this later... Hacky quick shit.
                for (idx, chunk) in help_str
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(1992)
                    .enumerate()
                {
                    let mut to_send = if idx == 0 {
                        String::new()
                    } else {
                        String::from("```\n")
                    };
                    let chunk_str: String = chunk.iter().collect();
                    to_send.push_str(chunk_str.as_str());
                    to_send.push_str("\n```");
                    ctx.say(to_send).await?;
                }
            }
        }
        None => {
            let mut key_vec: Vec<String> = data.file_map.keys().cloned().collect();
            key_vec.sort();
            let mut help_str = String::from("Quip categories:\n");
            for key in key_vec {
                help_str.push_str(format!("**{}**\n", key).as_str());
            }
            ctx.say(help_str).await?;
        }
    };
    Ok(())
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

/// Aliases: "!r" and "!rand." Play a random quip from all available quips or play a random quip from a specified category.
#[poise::command(prefix_command, guild_only = true, aliases("r", "rand"))]
async fn random(
    ctx: Context<'_>,
    #[description = "Quip category"] cat: Option<String>,
) -> Result<(), Error> {
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
        if let Some(_cat) = cat {
            let file_vec = data.get_vec(&_cat)?;
            idx = rng.random_range(0..file_vec.len());
            chosen_file = &file_vec[idx];
            chosen_category = _cat;
        } else {
            (chosen_file, chosen_category, idx) =
                data.get_from_global_index(rng.random_range(0..data.map_len))?;
        }
    };
    ctx.say(format!(
        "Playing quip \"{} {}\" ({})",
        chosen_category,
        // Convert to 1-based indexing.
        idx as u32 + 1,
        chosen_file.file_name().into_string().unwrap()
    ))
    .await?;
    play(&ctx, chosen_file).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let top_dir = if args.len() < 2 {
        String::from("audio")
    } else if args.len() == 2 {
        args[1].to_string()
    } else {
        panic!("Provide a single argument, the path to the directory containing audio files.")
    };

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    let data = Data::new(top_dir);

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

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: prefix_framework_options,
            commands: vec![list(), random(), disconnect(), help(), command],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;
    client.unwrap().start().await.unwrap();
}
