use teloxide::utils::command::BotCommands;
use teloxide::prelude::*;
use anyhow::Result;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "test command")]
    Test,
    #[command(description = "Get top n apys")]
    Top(usize),
}


pub async fn handle_commands(bot:Bot,msg:Message,cmd:Command)->Result<()>{
    match cmd {
        Command::Help=>{
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        },
        Command::Test => {
            handle_test(bot, msg).await?;
        },
        Command::Top(n)=>{
            bot.send_message(msg.chat.id, format!("you want top: {n}")).await?;
        }
    };
    Ok(())
}

async fn handle_test(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, "test works").await?;
    Ok(())
}