use serde_json::Value;
use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[description(
    "Translates a given text into the target language given as the first argument. \
        You can optionally prefix the source language as first argument, \
        otherwise it will be auto detected."
)]
#[example("en こんにちは！")]
#[example("de en Guten Abend!")]
pub fn translate(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let first_arg = args.single::<String>().unwrap();
    let second_arg = args.single::<String>().unwrap();

    // Get the target lang (or source lang if second language is given)
    let mut target_lang = match validate_unit(&first_arg) {
        Some(lang) => lang,
        None => {
            return Err(CommandError(
                "The first argument must be a valid target language code!".to_string(),
            ))
        }
    };

    // Try to grab a second language parameter. On success, use that as the target_lang and the
    // initial first parameter as source language (i.e. switch from <target> <text> to <source> <target> <text>)
    let mut source_lang = match validate_unit(&second_arg) {
        Some(lang) => {
            // When the second argument is a language,
            // swap first and second arguments
            let target_lang_copy = target_lang;
            target_lang = lang;
            target_lang_copy
        }
        None => {
            // Else write this argument back to args, as it's part of the translation string!
            args.rewind();
            "auto"
        }
    };

    // If nothing is left to translate, a text is missing
    if args.is_empty() {
        return Err(CommandError(
            "There needs to be a text to be translated!".to_string(),
        ));
    }

    let text = args.rest();
    let client = reqwest::blocking::Client::new();

    // Send the query and parse it as text response
    let response = client
        .get(
            format!(
            "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
            source_lang, target_lang, text
        )
            .as_str(),
        )
        .send()?
        .text()?;

    // Get loosely typed json format
    let json: Value = serde_json::from_str(&response)?;

    // Join translated sentences into one output string
    let data_array = json[0].as_array().unwrap();
    let translated_sentences = data_array
        .iter()
        .fold(String::default(), |translation, data| {
            let transl_sentence = data[0].as_str().unwrap();
            translation + transl_sentence
        });

    // Get recognized source language from response
    source_lang = json[2].as_str().unwrap();

    // Send message with translation
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(format!(
                "Translation from {} -> {}",
                source_lang.to_ascii_uppercase(),
                target_lang.to_ascii_uppercase()
            ))
            .description(translated_sentences)
        })
    });

    Ok(())
}

fn validate_unit(unit_arg: &String) -> Option<&str> {
    let unit_lowercase = unit_arg.to_ascii_lowercase();

    match unit_lowercase.as_str() {
        "ja" | "jp" | "japanese" => Some("ja"),
        "fr" | "french" | "baguette" => Some("fr"),
        "ko" | "korean" | "kpop" => Some("ko"),
        "en" | "english" => Some("en"),
        "de" | "german" => Some("de"),
        "it" | "italian" => Some("it"),
        "es" | "spanish" => Some("es"),
        "ru" | "russian" => Some("ru"),
        _ => None,
    }
}
