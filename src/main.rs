mod commands;
mod core;

use crate::core::context::*;
use crate::core::util::send_error_msg;
use chrono::Utc;
use log::{error, info};
use serenity::{
    async_trait,
    framework::standard::{macros::hook, CommandResult, DispatchError, Reason, StandardFramework},
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
            Some(Activity::listening("~help, mio help")),
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
    env_logger::builder()
        .filter_module("serenity", log::LevelFilter::Error)
        .filter_module("tracing", log::LevelFilter::Error)
        .init();

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
                .prefixes(vec!["~", "mio "])
                .owners(owners)
        })
        .on_dispatch_error(dispatch_error)
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
        error!("Client error: {:?}", why);
    }
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::CheckFailed(check_name, reason) => {
            info!(
                "Command check '{}' failed. Reason: {:?}",
                check_name, reason
            );

            if let Reason::User(error_msg) = reason {
                send_error_msg(ctx, msg, None, &error_msg).await;
            };
        }
        DispatchError::NotEnoughArguments { min, given } => {
            info!(
                "Command didn't receive enough arguments. Message: '{}', Given: {} Expected: {}",
                msg.content, given, min
            );

            send_error_msg(
                ctx,
                msg,
                None,
                format!(
                    "Command needs at least {} arguments. See ~help <command> for guidance.",
                    min
                )
                .as_str(),
            )
            .await;
        }
        _ => (),
    }
}

#[hook]
async fn after(ctx: &Context, msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => info!(
            "Command '{}' processed message: {}",
            command_name, msg.content
        ),
        Err(error) => {
            info!(
                "Command '{}' returned error. Message: {}, Error: {:?}",
                command_name, msg.content, error
            );

            send_error_msg(ctx, msg, None, &error.to_string()).await;
        }
    }
}
