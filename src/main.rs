#[macro_use] extern crate serenity;
#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate dotenv;

use std::env;
use serenity::prelude::EventHandler;
use serenity::model::gateway::{Game, Ready};
use serenity::prelude::Context;
use dotenv::dotenv;

#[derive(Deserialize)]
struct Translation {
    lang: String,
    code: u16,
    text: Vec<String>
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, context: Context, _: Ready) {
        println!("Bot online!");

        context.set_game(Game::playing("?thelp | ?tr"));
    }
}


fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("token");

    let mut client = serenity::client::Client::new(&token, Handler).unwrap();
    client.with_framework(serenity::framework::standard::StandardFramework::new()
        .configure(|c| c.prefix("?t"))

        .cmd("r", translate_message)
        .cmd("help", help)
        .cmd("langs", langs)
    );

    if let Err(e) = client.start() {
        println!("An error occured: {:?}", e);
    }
}

fn translate(text: &str, lang: &str) -> String
{
    let key = env::var("YANDEX_KEY").expect("yandex key");
    let url = format!("https://translate.yandex.net/api/v1.5/tr.json/translate?key={}&text={}&lang={}", key, text.replace("#", "%23"), lang);

    let res = reqwest::get(&url);
    if let Ok(mut response) = res
    {
        let ret: Translation = response.json().unwrap();
        return ret.text.join(" ");
    }
    return String::new();
}

command!(translate_message(_context, message) {
    let new_content = &message.content[4..];
    let mut lang = String::from("en");
    let mut proc_content = String::from("\u{200b}");

    for seg in new_content.split_whitespace() {

        if seg.starts_with("d-") {
            lang = seg.replace("d-", "");
        }
        else {
            proc_content.push_str(seg);
            proc_content.push_str(" ");
        }
    }

    let _ = message.reply(&translate(proc_content.trim(), &lang));
});

command!(help(_context, message) {
    let _ = message.channel_id.send_message(|m| {
        m.embed(|e| {
            e.title("Help")
            .description("
            `?thelp` - Get this page

            `?tr <text> d-[lang]` - Translate text; example:

```
>> ?tr Hello world! d-es
<< @JellyWX: Hola mundo!```

            `?tlangs` - Get a list of all supported languages with codes
            ")
        })
    });
});

command!(langs(_context, message) {
    let _ = message.channel_id.say("A full list of languages and codes is available here: https://gist.github.com/JellyWX/f1d83c6966c93c83c126affd2640886a");
});
