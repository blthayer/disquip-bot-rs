use poise::serenity_prelude as serenity;
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Responds "world" to "hello"
#[poise::command(prefix_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("world").await?;
    Ok(())
}

/// Joins the user's voice channel and plays the AOE 1 "yes" taunt.
#[poise::command(prefix_command, guild_only = true)]
async fn yes(ctx: Context<'_>) -> Result<(), Error> {
    // Get user's voice channel.
    let guild = ctx.guild().unwrap().to_owned();
    let user_id = ctx.author().id;
    let voice_states = guild.voice_states.get(&user_id);
    match voice_states {
        Some(voice_states) => {
            ctx.reply(format!(
                "User's voice channel id is {:?}",
                voice_states.channel_id
            ))
            .await?;
        }
        None => {
            ctx.reply("You must be in a voice channel!".to_string())
                .await?;
        }
    }

    // Load up the file!
    // let file = songbird::input::File::new("yes.mp3");
    // ctx.say("world").await?;
    Ok(())
}

/// Show help menu
#[poise::command(prefix_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "Type `!help command` for more info on a command.",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_VOICE_STATES;

    let prefix_framework_options = poise::PrefixFrameworkOptions {
        prefix: Some("!".to_string()),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: prefix_framework_options,
            commands: vec![hello(), help(), yes()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
