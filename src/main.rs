mod commands;
mod core;

use crate::core::constants::ERROR_COLOR;
use crate::core::context::*;
use chrono::Utc;
use log::info;
use serenity::{
    async_trait,
    framework::standard::{macros::hook, CommandResult, StandardFramework},
    http::Http,
    model::{channel::Message, event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};
use sysinfo::{System, SystemExt};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        use serenity::model::gateway::Activity;
        use serenity::model::user::OnlineStatus;

        ctx.set_presence(
            Some(Activity::listening("~help, aoyama help")),
            OnlineStatus::Online,
        )
        .await
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[tokio::main]
async fn main() {
    kankyo::load(false).expect("Failed to load .env file");
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = Http::new_with_token(&token);

    let app_info = match http.get_current_application_info().await {
        Ok(info) => info,
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let bot_user = match http.get_current_user().await {
        Ok(user) => user,
        Err(why) => panic!("Could not access the bot id: {:?}", why),
    };

    let mut owners = HashSet::new();
    if let Some(team) = &app_info.team {
        owners.insert(team.owner_user_id);
    } else {
        owners.insert(app_info.owner.id);
    }

    let framework = StandardFramework::new()
        .configure(|c| {
            c.on_mention(Some(bot_user.id))
                .prefixes(vec!["~", "aoyama "])
                .owners(owners)
        })
        .after(after)
        .group(&commands::web::WEB_GROUP)
        .group(&commands::fun::FUN_GROUP)
        .group(&commands::system::SYSTEM_GROUP)
        .group(&commands::moderation::MODERATION_GROUP)
        .group(&commands::nsfw::NSFW_GROUP)
        .help(&commands::help::HELP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<StartTimeContainer>(Utc::now());
        data.insert::<SysInfoContainer>(System::new_all());
        data.insert::<AppInfoContainer>(app_info);
        data.insert::<BotUserContainer>(bot_user);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[hook]
async fn after(ctx: &Context, msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!(
            "Command '{}' processed message: {}",
            command_name, msg.content
        ),
        Err(error) => {
            println!(
                "Command '{}' returned error. Message: {}, Error: {:?}",
                command_name, msg.content, error
            );

            let _ = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| e.colour(ERROR_COLOR).title("Oh noes!").description(error))
                })
                .await;
        }
    }
}
