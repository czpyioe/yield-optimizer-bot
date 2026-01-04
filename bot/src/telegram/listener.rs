use teloxide::prelude::*;
use anyhow::Result;
use crate::strategy::logic::StrategyEngine;
use crate::telegram::client::TelegramBot;
use crate::telegram::commands;

pub async fn run(telegram_bot: TelegramBot, engine: StrategyEngine) -> Result<()> {
    println!("Telegram listener started");
    
    let bot = telegram_bot.get_bot();
    
    let handler = Update::filter_message()
        .endpoint(move |msg: Message| {
            let eng = engine.clone();
            let tg_bot = telegram_bot.clone();
            async move {
                commands::handle_message(tg_bot, msg, eng).await
            }
        });

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}