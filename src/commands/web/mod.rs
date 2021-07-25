use serenity::framework::standard::macros::group;

mod convert;
mod tldr;
mod translate;
mod weather;

use self::convert::CONVERT_COMMAND;
use self::tldr::TLDR_COMMAND;
use self::translate::TRANSLATE_COMMAND;
use self::weather::WEATHER_COMMAND;

#[group]
#[commands(convert, weather, tldr, translate)]
struct Web;
