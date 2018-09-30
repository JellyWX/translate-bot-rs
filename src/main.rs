#[macro_use] extern crate serenity;
#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate dotenv;

use std::env;
use serenity::prelude::EventHandler;
use dotenv::dotenv;

#[derive(Deserialize)]
struct Translation {
    lang: String,
    code: u16,
    text: Vec<String>
}

struct Handler;

impl EventHandler for Handler {}


fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("token");

    let mut client = serenity::client::Client::new(&token, Handler).unwrap();
    client.with_framework(serenity::framework::standard::StandardFramework::new()
        .configure(|c| c.prefix("?t"))
        .cmd("r", translate_message)
        .cmd("help", help)
    );

    if let Err(e) = client.start() {
        println!("An error occured: {:?}", e);
    }
}

fn translate(text: &str, lang: &str) -> String
{
    let key = env::var("YANDEX_KEY").expect("yandex key");
    let url = format!("https://translate.yandex.net/api/v1.5/tr.json/translate?key={}&text={}&lang={}", key, text.replace("#", "%23"), lang);
    println!("{}", url);

    let res = reqwest::get(&url);
    if let Ok(mut response) = res
    {
        let ret: Translation = response.json().unwrap();
        println!("{}", ret.lang);
        return ret.text.join(" ");
    }
    return "".to_owned()
}

command!(translate_message(_context, message) {
    let new_content = &message.content[4..];
    let mut lang = String::from("en");
    let mut proc_content = String::new();

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
    let _ = message.channel_id.send_message(|m| {m.content("Help")});
});
