use std::{
    collections::HashMap,
    env,
    fs::{DirEntry, read_dir},
};

use poise::serenity_prelude as serenity;
use songbird::SerenityInit;
type FileMap = HashMap<String, Vec<DirEntry>>;
struct Data {
    pub file_map: FileMap,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::PrefixContext<'a, Data, Error>;
type GenericContext<'a> = poise::Context<'a, Data, Error>;

/// Play a quip!
#[poise::command(prefix_command, guild_only = true, hide_in_help = true)]
async fn join_and_play(
    ctx: Context<'_>,
    #[description = "Quip number"] num: usize,
) -> Result<(), Error> {
    // Get user's voice channel.
    let guild = ctx.guild().unwrap().to_owned();
    let user_id = ctx.author().id;
    let voice_states = guild.voice_states.get(&user_id);

    let Some(voice_states) = voice_states else {
        ctx.reply("You must be in a voice channel!".to_string())
            .await?;
        return Ok(());
    };

    let Some(channel_id) = voice_states.channel_id else {
        ctx.reply("Somehow there's no channel_id, which does not make sense...".to_string())
            .await?;
        return Ok(());
    };

    let command = ctx.invoked_command_name();
    let file_vec = ctx.data().file_map.get(ctx.invoked_command_name());
    let chosen_file: &DirEntry;
    match file_vec {
        Some(file_vec) => {
            let attempt_chosen_file = file_vec.get(num - 1);
            match attempt_chosen_file {
                Some(_chosen_file) => chosen_file = _chosen_file,
                None => {
                    ctx.reply(format!("The given integer \"{:?}\" is invalid. Valid integers for the \"{:?}\" command range from 1 to {:?}", num, command, file_vec.len()))
                    .await?;
                    return Ok(());
                }
            }
        }
        None => {
            ctx.reply(format!(
                "The given command \"{:?}\" is not valid, see \"!help\"",
                command
            ))
            .await?;
            return Ok(());
        }
    }
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let songbird_id = songbird::id::ChannelId::from(channel_id);
    // It seems to be fine if there are multiple join calls, probably no need
    // to add our own conditional here.
    if let Ok(handler_lock) = manager.join(guild.id, songbird_id).await {
        let mut handler = handler_lock.lock().await;

        let file = songbird::input::File::new(chosen_file.path());
        handler.play_only_input(file.into());
    }

    Ok(())
}

/// Show help menu
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

fn get_file_map(top_dir: String) -> FileMap {
    let mut map: HashMap<String, Vec<DirEntry>> = HashMap::new();

    let result = read_dir(top_dir).unwrap();
    for r in result {
        let u = r.unwrap();
        if u.file_type().unwrap().is_dir() {
            let files = read_dir(u.path()).unwrap();
            for f in files {
                let _f = f.unwrap();
                let key = u.file_name().into_string().unwrap();
                match map.entry(key) {
                    std::collections::hash_map::Entry::Occupied(mut oe) => {
                        oe.get_mut().push(_f);
                    }
                    std::collections::hash_map::Entry::Vacant(ve) => {
                        ve.insert(vec![_f]);
                    }
                }
            }
        }
    }

    // Sort.
    for val in map.values_mut() {
        val.sort_by_key(|a| a.path());
    }
    map
}

/// List available quip categories or list available quips for a given command.
#[poise::command(prefix_command, guild_only = true)]
async fn list(
    ctx: Context<'_>,
    #[description = "Quip category"] cat: Option<String>,
) -> Result<(), Error> {
    let file_map = &ctx.data().file_map;
    match cat {
        Some(cat) => {
            if let Some(cat_vec) = file_map.get(&cat) {
                let mut help_str = format!("Available quips for category \"{}\":\n", cat);
                for (idx, item) in cat_vec.iter().enumerate() {
                    help_str.push_str(
                        format!(
                            "**{}**: {:?}\n",
                            idx - 1,
                            item.file_name().into_string().unwrap()
                        )
                        .as_str(),
                    );
                }
                ctx.reply(help_str).await?;
            } else {
                ctx.reply("The provided category is invalid. Use \"!list\" with no arguments to get valid categories.").await?;
                return Ok(());
            }
        }
        None => {
            let mut key_vec: Vec<String> = file_map.keys().cloned().collect();
            key_vec.sort();
            let mut help_str = String::from("Quip categories:\n");
            for key in key_vec {
                help_str.push_str(format!("**{}**\n", key).as_str());
            }
            ctx.reply(help_str).await?;
        }
    };
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

    let file_map = get_file_map(top_dir);

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_VOICE_STATES;

    let prefix_framework_options = poise::PrefixFrameworkOptions {
        prefix: Some("!".to_string()),
        ..Default::default()
    };

    let mut command = join_and_play();
    let mut keys: Vec<_> = file_map.keys().cloned().collect();
    keys.sort();
    command.aliases = file_map.keys().cloned().collect();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: prefix_framework_options,
            commands: vec![list(), help(), command],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { file_map })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;
    client.unwrap().start().await.unwrap();
}
