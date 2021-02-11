use std::env;
use ksoft::{
    blocking::{
        Client,
        EventHandler as KSoftEvents,
    },
    model::bans::BanUpdate,
    prelude::SpanType
};

fn main() {
    let ksoft = Client::new(env::var("KSOFT_TOKEN").unwrap());
    ksoft.event_handler(EventHandler);

    if let Ok(res) = ksoft.images.random_reddit("some subreddit", true, SpanType::Month) {
        match res {
            Ok(reddit) => {
                println!("Reddit image received! {:#?}", reddit);
            },
            Err(why) => {
                println!("Got an error! {}", why.message);
            }
        }
    }
}


struct EventHandler;

impl KSoftEvents for EventHandler {
    fn ban_updated(&self, data: Vec<BanUpdate>) {
        println!("Ban updated received! data: {:#?}", data);
    }
}