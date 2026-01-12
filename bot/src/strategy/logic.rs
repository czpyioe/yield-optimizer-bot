// use sqlx::PgPool;
// use anyhow::Result;
// use std::env;

// use crate::db::models::ApySnapshot;
// use crate::telegram::{self, client::TelegramBot};
// use crate::db::queries;


// #[derive(Clone)]
// pub struct StrategyEngine {
//     db_pool:PgPool,
//     telegram_bot:TelegramBot,
// }


// impl StrategyEngine{
//     pub fn new(db_pool:PgPool,telegram_bot:TelegramBot)->Self{
//         Self {
//             db_pool: db_pool,
//             telegram_bot: telegram_bot, 
//         }
//     }


//     pub async fn get_top_yields(&self, limit: usize) -> Result<Vec<ApySnapshot>> {
//         let results = queries::get_recent_apys(&self.db_pool).await?;
//         Ok(results.into_iter().take(limit).collect())
//     }


//     async fn get_best_apy(&self)->Result<Vec<ApySnapshot>>{
//         let result = queries::get_recent_apys(&self.db_pool).await?;
//         Ok(result.into_iter().take(3).collect())
//     }

//     // fn find_rebalancing_opportunities(&self)->Result<()>{
//     //     Ok(())
//     // }


//     // fn find_high_yield_opportunities(&self)->Result<()>{

//     //     Ok(())
//     // }


//     // fn detect_apy_drops(&self)->Result<()>{
//     //     Ok(())
//     // }

//     fn format_opportunities(&self, opportunities: Vec<ApySnapshot>) -> String {
//         let mut message = "*Top 3 Yields Right Now*\n\n".to_string();
        
//         for (i, snapshot) in opportunities.iter().enumerate() {
//             let apy = snapshot.apy.unwrap_or(0.0);
//             message.push_str(&format!(
//                 "{}. {} - {} on {}: *{:.2}%*\n",
//                 i + 1,
//                 snapshot.asset,
//                 snapshot.protocol,
//                 snapshot.network,
//                 apy
//             ));
//         }
        
//         message
//     }

//     pub fn format_yields(&self, opportunities: &[ApySnapshot]) -> String {
//         let mut message = "*Top Yields:*\n\n".to_string();
        
//         for (i, snapshot) in opportunities.iter().enumerate() {
//             let apy = snapshot.apy.unwrap_or(0.0);
//             message.push_str(&format!(
//                 "{}. {} - {} on {}: *{:.2}%*\n",
//                 i + 1,
//                 snapshot.asset,
//                 snapshot.protocol,
//                 snapshot.network,
//                 apy
//             ));
//         }
//         message
//     }


//     pub async fn send_message(&self, text: &str) -> Result<()> {
//         self.telegram_bot.send_message(text).await
//     }
// }