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
