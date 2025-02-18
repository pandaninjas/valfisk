use anyhow::{anyhow, Result};
use poise::serenity_prelude as serenity;

use crate::Context;

/// Time yourself out for a specific duration
#[poise::command(rename = "self-timeout", slash_command)]
pub async fn self_timeout(
    ctx: Context<'_>,
    #[description = "The duration to time yourself out for"] duration: String,
) -> Result<()> {
    ctx.defer_ephemeral().await?;
    let duration = humantime::parse_duration(&duration);

    if let Ok(duration) = duration {
        // this is a bit weird; `ctx.author_member()` exists but it isn't mutable
        let member = ctx
            .guild_id()
            .ok_or_else(|| anyhow!("Unable to obtain guild"))?
            .member(&ctx, ctx.author().id)
            .await
            .ok();

        if let Some(mut member) = member {
            let start = chrono::Utc::now();
            let end = start + duration;
            let end_serenity = serenity::Timestamp::from_unix_timestamp(end.timestamp())?;

            member
                .disable_communication_until_datetime(&ctx, end_serenity)
                .await?;

            ctx.send(
                poise::CreateReply::new().embed(
                    serenity::CreateEmbed::new()
                        .title("Self-timeout in effect")
                        .field("Start", format!("<t:{0}:F>", start.timestamp()), false)
                        .field(
                            "End",
                            format!("<t:{0}:F> (<t:{0}:R>)", end.timestamp()),
                            false,
                        )
                        .color(0x4ade80),
                ),
            )
            .await?;
        } else {
            ctx.say("Error: Member unavailable!").await?;
        }
    } else {
        ctx.say("Error: Invalid duration!").await?;
    }

    Ok(())
}
