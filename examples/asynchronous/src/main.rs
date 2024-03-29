use std::env;
use ksoft::{
    Client,
    EventHandler as KSoftEvents,
    model::bans::BanUpdate,
    prelude::{SpanType, async_trait}
};

#[tokio::main]
async fn main() {
    let ksoft = Client::new(env::var("KSOFT_TOKEN").unwrap());
    ksoft.event_handler(EventHandler);

    if let Ok(res) = ksoft.images.random_reddit("some subreddit", true, SpanType::Month).await {
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

#[async_trait]
impl KSoftEvents for EventHandler {
    async fn ban_updated(&self, data: Vec<BanUpdate>) {
        println!("Ban updated received! data: {:#?}", data);
    }
}
