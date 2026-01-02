use teloxide::prelude::{Bot,ChatId, Requester};
use anyhow::Result;


pub struct TelegramBot{
    bot: Bot
}

impl TelegramBot {
    pub fn new(token:String)->Self{
        Self { 
            bot: Bot::new(token)
        }
    }

    pub async fn start(self)->Result<()>{
    
        Ok(())
    }

    pub async fn send_message(&self,chat_id:i64,text:&str)->Result<()>{
        let chat_id = ChatId(chat_id);
        self.bot.send_message(chat_id, text).await?;
        Ok(())
    }
}