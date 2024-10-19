use teloxide::{
    prelude::*,
    dispatching::dialogue::InMemStorage
};

type EntropyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[Default]
    Start,
    RecivePassword
}

pub async fn start_dialogue(
    bot: Bot, 
    dialogue: EntropyDialogue, 
    msg: Message
) -> HandlerResult {
    bot.send_message(msg.chat.id, "What's your password?").await?;
    dialogue.update(State::RecivePassword).await?;
    Ok(())
}

async fn recive_password(
    bot: Bot, 
    dialogue: MyDialogue, 
    msg: Message
) -> HandlerResult {
    match msg.text() {
        Some(password) => {
            bot.send_message(msg.chat.id, password).await?;
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain password.")
        }
    }

    Ok(())
}
