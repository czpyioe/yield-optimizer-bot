use teloxide::prelude::*;

use crate::telegram::handlers::commands::Command;
use crate::telegram::handlers::commands::handle_commands;

pub async fn run(bot:Bot) {
    let handler = Update::filter_message()
        .branch(
        dptree::entry()
            .filter_command::<Command>()
            .endpoint(handle_commands)
        );
    Dispatcher::builder(bot, handler)
        .build()
        .dispatch()
        .await;
}