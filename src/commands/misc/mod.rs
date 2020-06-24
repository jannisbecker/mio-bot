use serenity::framework::standard::macros::group;

mod collect_images;
mod convert;
mod say;
mod translate;
mod weather;

use self::collect_images::COLLECT_COMMAND;
use self::convert::CONVERT_COMMAND;
use self::say::SAY_COMMAND;
use self::say::YELL_COMMAND;
use self::translate::TRANSLATE_COMMAND;
use self::weather::WEATHER_COMMAND;

#[group]
#[commands(convert, say, yell, weather, translate, collect)]
struct Misc;
