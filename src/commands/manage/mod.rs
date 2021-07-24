use serenity::framework::standard::macros::group;

mod boosts;
mod fetch;
mod serverlist;

use self::boosts::LIST_BOOSTS_COMMAND;
use self::fetch::FETCH_COMMAND;
use self::serverlist::ADD_SERVER_COMMAND;

#[group]
#[commands(fetch, add_server, list_boosts)]
struct Manage;
