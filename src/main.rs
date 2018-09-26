#[macro_use] extern crate serenity;
#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate reqwest;

use std::env;
use serenity::prelude::EventHandler;

#[derive(Deserialize)]
struct Translation {
    lang: String,
    code: u16,
    text: Vec<String>
}

struct Handler;

impl EventHandler for Handler {}


fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token");

    let mut client = serenity::client::Client::new(&token, Handler).unwrap();
    client.with_framework(serenity::framework::standard::StandardFramework::new()
        .configure(|c| c.prefix("?t"))
        .cmd("r", translate_message)
    );

    if let Err(e) = client.start() {
        println!("An error occured: {:?}", e);
    }
}

fn translate(text: &str) -> String
{
    let key = env::var("YANDEX_KEY").expect("yandex key");
    let url = format!("https://translate.yandex.net/api/v1.5/tr.json/translate?key={}&text={}&lang=es", key, text);
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
    let new_content = &message.content[3..];

    let _ = message.reply(&translate(new_content));
});
