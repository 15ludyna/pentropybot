use std::error::Error;
use teloxide::prelude::*;

pub mod handlers;
pub mod keyboards;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bot = Bot::from_env();
    let handler = dptree::entry()
        .branch(Update::filter_message()
            .filter_command::<handlers::Command>()
            .endpoint(handlers::command_handler)
        )
        .branch(Update::filter_message().endpoint(handlers::keyboard_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
