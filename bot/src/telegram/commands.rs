use teloxide::prelude::*;
use anyhow::Result;
use crate::{strategy::logic::StrategyEngine, telegram::client::TelegramBot};

pub async fn handle_message(telegram_bot: TelegramBot,msg: Message,engine: StrategyEngine) -> Result<()> {
    
    if let Some(text) = msg.text() {
        let response = match text {
            "/help" => {
                "*Yield Optimizer Bot*\n\n\
                 Commands:\n\
                 /best - Top 3 yields\n\
                 /top5 - Top 5 yields\n\
                 /help - Show this message".to_string()
            }
            "/best" => {
                match engine.get_top_yields(3).await {
                    Ok(yields) => engine.format_yields(&yields),
                    Err(e) => format!("Error: {}", e),
                }
            }
            "/top5" => {
                match engine.get_top_yields(5).await {
                    Ok(yields) => engine.format_yields(&yields),
                    Err(e) => format!("Error: {}", e),
                }
            }
            _ => "Unknown command. Try /help".to_string(),
        };

        telegram_bot.send_message(&response).await?;
    }
    
    Ok(())
}