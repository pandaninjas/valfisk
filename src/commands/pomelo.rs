use anyhow::Result;
use poise::{serenity_prelude as serenity, CreateReply};

use crate::Context;

/// Get information on the username migration within the server
#[poise::command(slash_command)]
pub async fn pomelo(ctx: Context<'_>) -> Result<()> {
    if let Some(guild) = ctx.guild_id() {
        ctx.defer().await?;

        let members = guild
            .members(&ctx, None, None)
            .await?
            .into_iter()
            .filter(|m| !m.user.bot)
            .collect::<Vec<serenity::Member>>();

        let mut nonmigrated_users: Vec<&serenity::UserId> = vec![];

        for member in &members {
            if member.user.discriminator.is_some() {
                nonmigrated_users.push(&member.user.id);
            };
        }

        let embed = serenity::CreateEmbed::new()
            .title("Username migration / Pomelo")
            .description(format!(
                "**{}/{}** migrated",
                members.len() - nonmigrated_users.len(),
                members.len(),
            ))
            .color(0x2dd4bf)
            .field(
                "Unmigrated users",
                if nonmigrated_users.is_empty() {
                    "None!".to_owned()
                } else {
                    nonmigrated_users
                        .into_iter()
                        .map(|u| format!("<@{u}>"))
                        .collect::<Vec<String>>()
                        .join(" ")
                },
                false,
            );

        ctx.send(CreateReply::new().embed(embed)).await?;
    } else {
        ctx.say("Guild unavailable").await?;
    }

    Ok(())
}
