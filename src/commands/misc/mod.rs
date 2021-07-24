use serenity::framework::standard::macros::group;

mod convert;
mod say;
mod tldr;
mod translate;
mod weather;

use self::convert::CONVERT_COMMAND;
use self::say::{SAY_COMMAND, YELL_COMMAND};
use self::tldr::TLDR_COMMAND;
use self::translate::TRANSLATE_COMMAND;
use self::weather::WEATHER_COMMAND;

#[group]
#[commands(convert, say, yell, weather, tldr, translate)]
struct Misc;
