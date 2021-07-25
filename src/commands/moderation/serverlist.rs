use crate::core::{constants::MAIN_COLOR, context::BotUserContainer, util::guild_icon_url};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Error;
use serde::Deserialize;

use serenity::{
    builder::CreateEmbed,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    futures::future::join_all,
    model::{
        channel::{Embed, Message},
        prelude::User,
    },
};

lazy_static! {
    // Regex to parse nhentai IDs from command input
    static ref INVITE_ID_REGEX: Regex = Regex::new(r"discord\.gg/(\w+)").unwrap();
}

#[command]
#[sub_commands(add_server, sort_servers)]
#[description = "Provides various sub-commands to moderate a list of servers.\nRefer to the sub-commands for more info."]
pub async fn serverlist() -> CommandResult {
    Ok(())
}

#[command("add")]
#[description("Creates a serverlist embed for the given server in the current channel")]
#[usage("<discord invite link>")]
#[example("https://discord.gg/gochiusa")]
async fn add_server(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let _ = msg.delete(&ctx).await;

    let invite_id = INVITE_ID_REGEX
        .captures(args.rest())
        .expect("Please supply a valid discord invite link")
        .get(1)
        .expect("Please supply a valid discord invite link")
        .as_str();

    let invite_info: InviteInfo = get_invite_info(invite_id).await?;

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| create_server_embed(e, &invite_info, &msg.author))
        })
        .await;

    Ok(())
}

#[command("sort")]
#[description("Sorts all serverlist embeds in this channel alphabetically")]
async fn sort_servers(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let _ = msg.delete(&ctx).await;

    let ctx_data = ctx.data.read().await;
    let bot_user = ctx_data
        .get::<BotUserContainer>()
        .expect("Couldn't get bot user from context.");

    let mut messages: Vec<Message> = msg
        .channel_id
        .messages(&ctx.http, |retriever| retriever.limit(100))
        .await?
        .into_iter()
        .filter(|msg| msg.author.id == bot_user.id && msg.embeds.len() > 0)
        .collect();

    let mut embeds: Vec<Embed> = messages.iter().map(|m| m.embeds[0].clone()).collect();

    embeds.sort_by_cached_key(|e| e.title.clone().unwrap_or_default());

    let futures = messages.iter_mut().enumerate().map(|(i, m)| {
        let embed = embeds.pop().unwrap();
        m.edit(&ctx, |m| m.set_embed(CreateEmbed::from(embed)))
    });

    join_all(futures).await;

    Ok(())
}

async fn get_invite_info(invite_id: &str) -> Result<InviteInfo, Error> {
    let client = reqwest::Client::new();
    client
        .get(format!(
            "https://discord.com/api/v9/invites/{}?with_counts=true&with_expiration=true",
            invite_id
        ))
        .send()
        .await?
        .json()
        .await
}

fn create_server_embed<'a>(
    e: &'a mut CreateEmbed,
    info: &InviteInfo,
    author: &User,
) -> &'a mut CreateEmbed {
    e.color(MAIN_COLOR)
        .title(&info.guild.name)
        .description(format!(
            "{}\n\
                        https://discord.gg/{}\n\n\
                        **{}** Members, **{}** Online",
            &info.guild.description.clone().unwrap_or_default(),
            &info.code,
            &info.approximate_member_count,
            &info.approximate_presence_count,
        ));

    if let Some(icon_id) = &info.guild.icon {
        e.thumbnail(guild_icon_url(&info.guild.id, icon_id, 64));
    }

    e.footer(|f| {
        if info.expires_at.is_some() {
            f.text(format!("{}   Expires on", &author.name));
        } else {
            f.text(&author.name);
        }

        if let Some(url) = author.avatar_url() {
            f.icon_url(url);
        }

        f
    });

    if let Some(date) = &info.expires_at {
        e.timestamp(date.as_str());
    }

    e
}

#[derive(Deserialize, Debug)]
struct InviteInfo {
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
