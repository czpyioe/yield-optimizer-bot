use teloxide::prelude::{Bot,ChatId, Requester};
use anyhow::Result;



#[derive(Clone)]
pub struct TelegramBot{
    bot: Bot,
    chat_id:i64
}

impl TelegramBot {
    pub fn new(token:String,chat_id:i64)->Self{
        Self { 
            bot: Bot::new(token),
            chat_id: chat_id
        }
    }


    pub async fn send_message(&self,text:&str)->Result<()>{
        let chat_id = ChatId(self.chat_id);
        self.bot.send_message(chat_id, text).await?;
        Ok(())
    }

    pub fn get_bot(&self) -> Bot {
        self.bot.clone()
    }

    pub fn get_chat_id(&self) -> i64 {
        self.chat_id
    }
}