use crate::core::checks::MODERATOR_CHECK;
use serenity::framework::standard::macros::group;

mod boosts;
mod fetch;
mod serverlist;

use self::boosts::BOOSTS_COMMAND;
use self::fetch::FETCH_COMMAND;
use self::serverlist::SERVERLIST_COMMAND;

#[group]
#[checks(Moderator)]
#[commands(fetch, boosts, serverlist)]
struct Moderation;
