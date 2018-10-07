#[macro_use] extern crate serenity;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate dotenv;
extern crate typemap;

use std::env;
use serenity::prelude::EventHandler;
use serenity::model::gateway::{Game, Ready};
use serenity::prelude::Context;
use dotenv::dotenv;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use typemap::Key;


lazy_static! {
    static ref YANDEX_KEY: String = env::var("YANDEX_KEY").expect("yandex key");
}

#[derive(Deserialize)]
struct Translation {
    #[serde(rename="lang")]
    _lang: String,

    #[serde(rename="code")]
    _code: u16,

    text: Vec<String>
}

struct RateLimiter;

impl Key for RateLimiter {
    type Value = HashMap<u64, Vec<u32>>;
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
    {
        let mut data = client.data.lock();
        data.insert::<RateLimiter>(HashMap::default());
    }

    if let Err(e) = client.start() {
        println!("An error occured: {:?}", e);
    }
}

fn translate(text: &str, lang: &str) -> String {
    let url = format!("https://translate.yandex.net/api/v1.5/tr.json/translate?key={}&text={}&lang={}", *YANDEX_KEY, text.replace("#", "%23"), lang);

    let res = reqwest::get(&url);
    if let Ok(mut response) = res {
        let ret: Translation = response.json().unwrap();
        return ret.text.join(" ");
    }
    return String::new();
}

fn time() -> u32 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();

    since_epoch.as_secs() as u32
}

command!(translate_message(ctx, message) {

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

    let content = proc_content.trim();
    let len = content.len() as u32;

    if len > 100 {
        let _ = message.reply("Please use fewer characters");

    } else {
        let t = time();
        let id = message.author.id.as_u64();

        let total_chars;
        {
            let mut data = ctx.data.lock();
            let mut limits = data.get_mut::<RateLimiter>().unwrap();

            let counter = limits.entry(*id).or_insert(vec![len, t]);

            if t - counter[1] > 86400 {
                counter[0] = len;
            } else {
                counter[0] += len;
            }

            total_chars = counter[0];
        }

        if total_chars < 1000 {
            let _ = message.reply(&translate(content, &lang));
        } else {
            let _ = message.reply("You have exceeded your daily limit. Please come back later.");
        }
    }
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
