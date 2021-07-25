use serenity::framework::standard::macros::group;

mod say;

use self::say::{SAY_COMMAND, YELL_COMMAND};

#[group]
#[commands(say, yell)]
struct Fun;
