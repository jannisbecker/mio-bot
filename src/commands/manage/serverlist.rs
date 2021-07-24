use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

lazy_static! {
    // Regex to parse nhentai IDs from command input
    static ref INVITE_ID_REGEX: Regex = Regex::new(r"discord\.gg/(\w+)").unwrap();
}

#[command]
#[min_args(1)]
pub async fn add_server(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let _ = msg.delete(&ctx).await;

    let client = reqwest::Client::new();

    let invite_id = INVITE_ID_REGEX
        .captures(args.rest())
        .expect("Please supply a valid discord invite link")
        .get(1)
        .expect("Please supply a valid discord invite link")
        .as_str();

    let server_info: InviteQueryResponse = client
        .get(format!(
            "https://discord.com/api/v9/invites/{}?with_counts=true&with_expiration=true",
            invite_id
        ))
        .send()
        .await?
        .json()
        .await?;

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(&server_info.guild.name).description(format!(
                    "{}\n\
                        https://discord.gg/{}\n\n\
                        **{}** Members, **{}** Online",
                    &server_info.guild.description.clone().unwrap_or_default(),
                    invite_id,
                    &server_info.approximate_member_count,
                    &server_info.approximate_presence_count,
                ));

                if let Some(icon_url) = &server_info.guild.icon {
                    e.thumbnail(format!(
                        "https://cdn.discordapp.com/icons/{}/{}.gif?size=64",
                        &server_info.guild.id, icon_url
                    ));
                }

                e.footer(|f| {
                    if server_info.expires_at.is_some() {
                        f.text(format!("{}   Expires on", &msg.author.name));
                    } else {
                        f.text(&msg.author.name);
                    }

                    if let Some(url) = msg.author.avatar_url() {
                        f.icon_url(url);
                    }

                    f
                });

                if let Some(date) = &server_info.expires_at {
                    e.timestamp(date.as_str());
                }

                e
            })
        })
        .await;

    Ok(())
}

// #[command]
// pub async fn remove_server(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
//     Ok(())
// }

#[derive(Deserialize, Debug)]
struct InviteQueryResponse {
    code: String,
    guild: InviteGuild,
    approximate_member_count: u32,
    approximate_presence_count: u32,
    expires_at: Option<String>,
}

#[derive(Deserialize, Debug)]
struct InviteGuild {
    id: String,
    name: String,
    splash: Option<String>,
    banner: Option<String>,
    description: Option<String>,
    icon: Option<String>,
}
