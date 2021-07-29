use crate::core::constants::MAIN_COLOR;
use crate::core::util::send_error_msg;
use chrono::{DateTime, Utc};
use serenity::futures::StreamExt;
use serenity::model::guild::Member;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[description(
    "Lists all members that are currently boosting the server \n\
    as well as the starting date of their boosting."
)]
pub async fn boosts(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut boosting_members: Vec<(Member, DateTime<Utc>)> = Vec::new();

    let mut guild_members = msg.guild_id.unwrap().members_iter(&ctx).boxed();

    while let Some(member_result) = guild_members.next().await {
        if let Ok(member) = member_result {
            if let Some(boost_date) = member.premium_since {
                boosting_members.push((member, boost_date))
            }
        }
    }

    if !boosting_members.is_empty() {
        let embed_str =
            boosting_members
                .into_iter()
                .fold(String::new(), |mut acc, (member, boost_date)| {
                    acc.push_str(
                        format!(
                            "**{}#{}** - {}",
                            member.user.name,
                            member.user.discriminator,
                            boost_date.format("%b %e %Y")
                        )
                        .as_str(),
                    );
                    acc
                });

        let _ = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.colour(MAIN_COLOR)
                        .title("Members boosting this server")
                        .description(embed_str)
                })
            })
            .await;
    } else {
        send_error_msg(
            ctx,
            msg,
            None,
            "Couldn't find any members boosting this server".into(),
        )
        .await;
    }

    Ok(())
}
