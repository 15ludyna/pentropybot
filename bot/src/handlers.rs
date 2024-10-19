use std::error::Error;
use teloxide::{
    prelude::*,
    utils::command::BotCommands,
};

#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Start) => {
                bot.send_message(msg.chat.id, "Hello").await?;
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found").await?;
            }
        }
    }

    Ok(())
}
