use std::error::Error;
use teloxide::prelude::*;

mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bot = Bot::from_env();
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handlers::message_handler));
    
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler().build()
        .dispatch()
        .await;
    Ok(())
}
