use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use serde::Deserialize;
use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    futures::TryFutureExt,
    model::channel::Message,
    prelude::Context,
};
use std::env;

use crate::core::constants::MAIN_COLOR;

#[command]
#[description("Retrieves the weather forecast at the given location")]
#[usage("<city name>")]
#[example("Berlin")]
#[example("Sri Lanka")]
#[example("New York")]
#[min_args(1)]
pub async fn weather(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let token = match env::var("OPEN_WEATHER_MAP_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            return Err(CommandError::from(
                "The bot owner didn't provide the OpenWeatherMap api key".to_string(),
            ))
        }
    };

    let client = reqwest::Client::new();

    // Get coordinates for given location
    let search_arg = args.rest();

    if search_arg.is_empty() {
        return Err(CommandError::from(
            "Please supply a valid city name as argument",
        ));
    }

    let location: LocationQueryResponse = client
        .get("http://api.openweathermap.org/data/2.5/weather")
        .query(&[("appid", &token), ("q", &search_arg.to_string())])
        .send()
        .and_then(|res| res.json())
        .await
        .map_err(|_| CommandError::from("There was an error parsing the weather api response"))?;

    let weather: WeatherQueryResponse = client
        .get("http://api.openweathermap.org/data/2.5/onecall")
        .query(&[
            ("appid", &token),
            ("lat", &location.coord.lat.to_string()),
            ("lon", &location.coord.lon.to_string()),
            ("units", &"metric".to_string()),
        ])
        .send()
        .await?
        .json()
        .await?;

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(MAIN_COLOR)
                    .title(format!("Weather in {}", search_arg))
                    .thumbnail(get_weather_image_url(&weather.current.weather[0].icon))
                    .description(format!(
                        "{} **{}** \n\
                        **Temp**: {:.0}°C (Feels like {:.0}°C)",
                        get_weather_emoji(&weather.current.weather[0].icon),
                        uppercase_first(&weather.current.weather[0].description),
                        &weather.current.temp,
                        &weather.current.feels_like
                    ))
                    .fields(vec![
                        (
                            "Weather",
                            format!(
                                "**Clouds**: {}% \n\
                                **Humidity**: {}% \n\
                                **Pressure**: {} hpa",
                                &weather.current.clouds,
                                &weather.current.humidity,
                                &weather.current.pressure
                            ),
                            true,
                        ),
                        (
                            "Wind",
                            format!(
                                "**Speed**: {}\n\
                                **Direction**: {}° ({})",
                                &weather.current.wind_speed,
                                weather.current.wind_deg,
                                format_direction(weather.current.wind_deg)
                            ),
                            true,
                        ),
                        (
                            "Location",
                            format!(
                                "**Sunrise**: {}\n\
                                **Sunset**: {}\n\
                                **Local Time**: {}",
                                format_timestamp(
                                    weather.current.sunrise,
                                    weather.timezone_offset,
                                    "%H:%M"
                                ),
                                format_timestamp(
                                    weather.current.sunset,
                                    weather.timezone_offset,
                                    "%H:%M"
                                ),
                                format_timestamp(
                                    weather.current.dt,
                                    weather.timezone_offset,
                                    "%H:%M, %b %e %Y"
                                ),
                            ),
                            false,
                        ),
                    ])
            })
        })
        .await;

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(MAIN_COLOR)
                    .title(format!("Forecast for {}", search_arg));

                for day_weather in &weather.daily[1..] {
                    e.field(
                        format!(
                            "{}",
                            format_timestamp(day_weather.dt, weather.timezone_offset, "%e %b %Y")
                        ),
                        format!(
                            "{} **{}** \n\
                        **Temp**: {:.0}°C\n\
                        **Humidity**: {}%",
                            get_weather_emoji(&day_weather.weather[0].icon),
                            uppercase_first(&day_weather.weather[0].description),
                            &day_weather.temp.day,
                            &day_weather.humidity
                        ),
                        true,
                    );
                }
                e
            })
        })
        .await;

    Ok(())
}

fn get_weather_image_url(code: &String) -> String {
    format!("http://openweathermap.org/img/wn/{}@2x.png", code)
}

fn get_weather_emoji(code: &String) -> String {
    match code.as_str() {
        "01d" => "☀️",
        "01n" => "🌕",
        "02d" | "02n" => "⛅",
        "03d" | "03n" => "☁️",
        "04d" | "04n" => "☁️",
        "09d" | "09n" => "🌧️",
        "10d" | "10n" => "🌧️",
        "11d" | "11n" => "🌩️",
        "13d" | "13n" => "❄️",
        "50d" | "50n" => "🌫️",
        _ => "☀️",
    }
    .to_string()
}

fn uppercase_first(s: &str) -> String {
    format!("{}{}", (&s[..1].to_string()).to_uppercase(), &s[1..])
}

// "%H:%M, %e %b %Y"
fn format_timestamp(timestamp: i64, offset: i32, format: &str) -> String {
    let date_time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
    let local_time = date_time.with_timezone(&FixedOffset::east(offset));

    local_time.format(format).to_string()
}

fn format_direction(degrees: i32) -> String {
    match degrees {
        x if x <= 45 || x > 315 => "North",
        x if x <= 135 => "East",
        x if x <= 225 => "South",
        x if x <= 315 => "West",
        _ => "",
    }
    .to_string()
}

#[derive(Deserialize, Debug)]
struct LocationQueryResponse {
    coord: Location,
    // Skip all the other data
}
#[derive(Deserialize, Debug)]
struct Location {
    lon: f64,
    lat: f64,
}

#[derive(Deserialize, Debug)]
struct WeatherQueryResponse {
    timezone_offset: i32,
    current: CurrentWeather,
    daily: Vec<DailyWeather>,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    dt: i64,
    sunrise: i64,
    sunset: i64,
    temp: f64,
    feels_like: f64,
    pressure: i32,
    humidity: i32,
    clouds: i32,
    visibility: Option<i32>,
    wind_speed: f64,
    wind_deg: i32,
    weather: Vec<Weather>,
}
#[derive(Deserialize, Debug)]
struct DailyWeather {
    dt: i64,
    temp: Temp,
    feels_like: FeelsLike,
    pressure: i32,
    humidity: i32,
    wind_speed: f64,
    wind_deg: i32,
    weather: Vec<Weather>,
}
#[derive(Deserialize, Debug)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}
#[derive(Deserialize, Debug)]
struct Temp {
    day: f64,
    min: f64,
    max: f64,
    night: f64,
    eve: f64,
    morn: f64,
}
#[derive(Deserialize, Debug)]
struct FeelsLike {
    day: f64,
    night: f64,
    eve: f64,
    morn: f64,
}
