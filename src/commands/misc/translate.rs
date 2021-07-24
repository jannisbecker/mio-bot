use serde_json::Value;
use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::channel::Message,
    prelude::Context,
};

use crate::core::constants::MAIN_COLOR;

#[command]
#[description(
    "Translates a given text into the target language given as the first argument. \
        You can optionally prefix the source language as first argument, \
        otherwise it will be auto detected."
)]
#[example("en こんにちは！")]
#[example("de en Guten Abend!")]
pub async fn translate(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first_arg = args.single::<String>()?;
    let second_arg = args.single::<String>()?;

    // Get the target lang (or source lang if second language is given)
    let mut target_lang = match validate_lang_arg(&first_arg) {
        Some(lang) => lang,
        None => {
            return Err(CommandError::from(
                "The first argument must be a valid two letter language code!",
            ))
        }
    };

    // Try to grab a second language parameter. On success, use that as the target_lang and the
    // initial first parameter as source language (i.e. switch from <target> <text> to <source> <target> <text>)
    let mut source_lang = match validate_lang_arg(&second_arg) {
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
        return Err(CommandError::from(
            "There needs to be a text to be translated!".to_string(),
        ));
    }

    let text = args.rest();
    let client = reqwest::Client::new();

    // Send the query and parse it as text response
    let response = client
        .get(
            format!(
            "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
            source_lang, target_lang, text
        )
            .as_str(),
        )
        .send()
        .await?
        .text()
        .await?;

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
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(MAIN_COLOR)
                    .title(format!(
                        "Translation from {} -> {}",
                        source_lang.to_ascii_uppercase(),
                        target_lang.to_ascii_uppercase()
                    ))
                    .description(translated_sentences)
            })
        })
        .await;

    Ok(())
}

fn validate_lang_arg(lang_arg: &String) -> Option<&str> {
    let unit_lowercase = lang_arg.to_ascii_lowercase();

    match unit_lowercase.as_str() {
        "afrikaans" | "af" => Some("af"),
        "albanian" | "sq" => Some("sq"),
        "amharic" | "am" => Some("am"),
        "arabic" | "ar" => Some("ar"),
        "armenian" | "hy" => Some("hy"),
        "azerbaijani" | "az" => Some("az"),
        "basque" | "eu" => Some("eu"),
        "belarusian" | "be" => Some("be"),
        "bengali" | "bn" => Some("bn"),
        "bosnian" | "bs" => Some("bs"),
        "bulgarian" | "bg" => Some("bg"),
        "catalan" | "ca" => Some("ca"),
        "cebuano" | "ceb" => Some("ceb"),
        "chinese" | "zh-CN" | "zh" => Some("zh"),
        "zh-TW" => Some("zh-TW"),
        "corsican" | "co" => Some("co"),
        "croatian" | "hr" => Some("hr"),
        "czech" | "cs" => Some("cs"),
        "danish" | "da" => Some("da"),
        "dutch" | "nl" => Some("nl"),
        "english" | "en" => Some("en"),
        "esperanto" | "eo" => Some("eo"),
        "estonian" | "et" => Some("et"),
        "finnish" | "fi" => Some("fi"),
        "french" | "fr" => Some("fr"),
        "frisian" | "fy" => Some("fy"),
        "galician" | "gl" => Some("gl"),
        "georgian" | "ka" => Some("ka"),
        "german" | "de" => Some("de"),
        "greek" | "el" => Some("el"),
        "gujarati" | "gu" => Some("gu"),
        "haitian" | "creole" | "ht" => Some("ht"),
        "hausa" | "ha" => Some("ha"),
        "hawaiian" | "haw" => Some("haw"),
        "hebrew" | "he" | "iw" => Some("iw"),
        "hindi" | "hi" => Some("hi"),
        "hmong" | "hmn" => Some("hmn"),
        "hungarian" | "hu" => Some("hu"),
        "icelandic" | "is" => Some("is"),
        "igbo" | "ig" => Some("ig"),
        "indonesian" | "id" => Some("id"),
        "irish" | "ga" => Some("ga"),
        "italian" | "it" => Some("it"),
        "japanese" | "jp" | "ja" => Some("ja"),
        "javanese" | "jv" => Some("jv"),
        "kannada" | "kn" => Some("kn"),
        "kazakh" | "kk" => Some("kk"),
        "khmer" | "km" => Some("km"),
        "kinyarwanda" | "rw" => Some("rw"),
        "korean" | "ko" => Some("ko"),
        "kurdish" | "ku" => Some("ku"),
        "kyrgyz" | "ky" => Some("ky"),
        "lao" | "lo" => Some("lo"),
        "latin" | "la" => Some("la"),
        "latvian" | "lv" => Some("lv"),
        "lithuanian" | "lt" => Some("lt"),
        "luxembourgish" | "lb" => Some("lb"),
        "macedonian" | "mk" => Some("mk"),
        "malagasy" | "mg" => Some("mg"),
        "malay" | "ms" => Some("ms"),
        "malayalam" | "ml" => Some("ml"),
        "maltese" | "mt" => Some("mt"),
        "maori" | "mi" => Some("mi"),
        "marathi" | "mr" => Some("mr"),
        "mongolian" | "mn" => Some("mn"),
        "myanmar" | "my" => Some("my"),
        "nepali" | "ne" => Some("ne"),
        "norwegian" | "no" => Some("no"),
        "nyanja" | "ny" => Some("ny"),
        "odia" | "or" => Some("or"),
        "pashto" | "ps" => Some("ps"),
        "persian" | "fa" => Some("fa"),
        "polish" | "pl" => Some("pl"),
        "portuguese" | "pt" => Some("pt"),
        "punjabi" | "pa" => Some("pa"),
        "romanian" | "ro" => Some("ro"),
        "russian" | "ru" => Some("ru"),
        "samoan" | "sm" => Some("sm"),
        "scots" | "gaelic" | "gd" => Some("gd"),
        "serbian" | "sr" => Some("sr"),
        "sesotho" | "st" => Some("st"),
        "shona" | "sn" => Some("sn"),
        "sindhi" | "sd" => Some("sd"),
        "sinhala" | "si" => Some("si"),
        "slovak" | "sk" => Some("sk"),
        "slovenian" | "sl" => Some("sl"),
        "somali" | "so" => Some("so"),
        "spanish" | "es" => Some("es"),
        "sundanese" | "su" => Some("su"),
        "swahili" | "sw" => Some("sw"),
        "swedish" | "sv" => Some("sv"),
        "tagalog" | "tl" => Some("tl"),
        "tajik" | "tg" => Some("tg"),
        "tamil" | "ta" => Some("ta"),
        "tatar" | "tt" => Some("tt"),
        "telugu" | "te" => Some("te"),
        "thai" | "th" => Some("th"),
        "turkish" | "tr" => Some("tr"),
        "turkmen" | "tk" => Some("tk"),
        "ukrainian" | "uk" => Some("uk"),
        "urdu" | "ur" => Some("ur"),
        "uyghur" | "ug" => Some("ug"),
        "uzbek" | "uz" => Some("uz"),
        "vietnamese" | "vi" => Some("vi"),
        "welsh" | "cy" => Some("cy"),
        "xhosa" | "xh" => Some("xh"),
        "yiddish" | "yi" => Some("yi"),
        "yoruba" | "yo" => Some("yo"),
        "zulu" | "zu" => Some("zu"),
        _ => None,
    }
}
