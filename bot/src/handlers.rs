use std::error::Error;
use teloxide::{
    prelude::*,
    utils::command::BotCommands,
    types::Me
};
use crate::keyboards::make_keyboard;

#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
}

pub async fn command_handler(
    bot: Bot,
    msg: Message,
    me: Me
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let keyboard = make_keyboard();
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Start) => {
                bot.send_message(msg.chat.id, "Hello")
                    .reply_markup(keyboard)
                    .await?;
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found")
                    .reply_markup(keyboard)
                    .await?;
            }
        }
    }

    Ok(())
}

pub async fn keyboard_handler(
    bot: Bot,
    msg: Message,
    me: Me
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match text {
            "Entropy" => {
                bot.send_message(msg.chat.id, "Entropy").await?;
            },
            "Info" => {
                bot.send_message(msg.chat.id, "Info").await?;
            },
            _ => {
                bot.send_message(msg.chat.id, "Command not found").await?;
            }
        }
    }

    Ok(())
}
