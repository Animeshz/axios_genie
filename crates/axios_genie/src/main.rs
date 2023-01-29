mod commands;

use poise::serenity_prelude as serenity;
use crate::serenity::GatewayIntents;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command, track_edits)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                edit_tracker: Some(poise::EditTracker::for_timespan(std::time::Duration::from_secs(3600))),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .client_settings(|client_builder| {
	    	client_builder
	    		.intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
	    		// ...
	    })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
