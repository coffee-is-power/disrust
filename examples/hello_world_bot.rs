use std::collections::HashSet;

use disrust::{Bot, Intent::*};


#[tokio::main]
async fn main() {
   let bot = Bot::new(HashSet::from([Guild, GuildMessages])).await;
   let token = option_env!("TOKEN");
   bot.login(token.unwrap()).await;
}
