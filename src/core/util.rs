use serenity::{client::Context, model::channel::Message};

use super::constants::ERROR_COLOR;

pub fn guild_icon_url(guild_id: &str, icon_id: &str, size: u16) -> String {
    if icon_id.starts_with("a_") {
        format!(
            "https://cdn.discordapp.com/icons/{}/{}.gif?size={}",
            guild_id, icon_id, size
        )
    } else {
        format!(
            "https://cdn.discordapp.com/icons/{}/{}.webp?size={}",
            guild_id, icon_id, size
        )
    }
}

pub async fn send_error_msg(ctx: &Context, msg: &Message, title: Option<&str>, error_msg: &str) {
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                if let Some(title_str) = title {
                    e.title(title_str);
                };

                e.colour(ERROR_COLOR).description(error_msg);

                e
            })
        })
        .await;
}
