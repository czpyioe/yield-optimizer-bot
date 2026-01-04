use sqlx::PgPool;
use anyhow::Result;
use std::env;

use crate::db::models::ApySnapshot;
use crate::telegram::{self, client::TelegramBot};
use crate::db::queries;



pub struct StartegyEngine {
    db_pool:PgPool,
    telegram_bot:TelegramBot,
    chat_id:i64
}


impl StartegyEngine{
    pub fn new(db_pool:PgPool,telegram_bot:TelegramBot,chat_id:i64)->Self{
        Self { 
            db_pool: db_pool, 
            telegram_bot: telegram_bot, 
            chat_id: chat_id
        }
    }


    pub async fn analyze_and_notify(&self)->Result<()>{
        let best_apy = self.get_best_apy().await?;
        println!("{:?}",best_apy);
        let message = self.format_opportunities(best_apy);
        self.send_alert(&message).await;
        Ok(())
    }


    async fn get_best_apy(&self)->Result<Vec<ApySnapshot>>{
        let result = queries::get_recent_apys(&self.db_pool).await?;
        Ok(result.into_iter().take(3).collect())
    }

    // fn find_rebalancing_opportunities(&self)->Result<()>{
    //     Ok(())
    // }


    // fn find_high_yield_opportunities(&self)->Result<()>{

    //     Ok(())
    // }


    // fn detect_apy_drops(&self)->Result<()>{
    //     Ok(())
    // }

    fn format_opportunities(&self, opportunities: Vec<ApySnapshot>) -> String {
        let mut message = "*Top 3 Yields Right Now*\n\n".to_string();
        
        for (i, snapshot) in opportunities.iter().enumerate() {
            let apy = snapshot.apy.unwrap_or(0.0);
            message.push_str(&format!(
                "{}. {} - {} on {}: *{:.2}%*\n",
                i + 1,
                snapshot.asset,
                snapshot.protocol,
                snapshot.network,
                apy
            ));
        }
        
        message
    }


    async fn send_alert(&self,message:&String)->Result<()>{
        self.telegram_bot.send_message(self.chat_id, message).await;
        Ok(())
    }
}