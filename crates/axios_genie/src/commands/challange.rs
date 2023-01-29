use poise::serenity_prelude::User;
use anyhow::Result;

use crate::{Context, Error};

#[derive(poise::ChoiceParameter)]
enum Platform {
    Codeforces,
    Typeracer,
}

#[poise::command(slash_command, prefix_command)]
async fn challange(
    ctx: Context<'_>,
    #[description = "Challange's platform"] platform: Platform,
    #[description = "Selected user"] user: User,
) -> Result<()> {
    match platform {
        Codeforces => codeforces::challange(ctx, user).await,
        Typeracer => typeracer::challange(ctx, user).await,
    }
}

mod codeforces {
    use super::*;

    pub async fn challange(
        ctx: Context<'_>,
        user: User,
    ) -> Result<()> {
        todo!()
    }
}

mod typeracer {
    use super::*;

    pub async fn challange(
        ctx: Context<'_>,
        user: User,
    ) -> Result<()> {
        todo!()
    }
}
