use disrust::{Bot, Intent::*};

#[tokio::main]
async fn main() {
    let bot = Bot::new(vec![Guild, GuildMessages]).await;
    let token = option_env!("TOKEN");
    bot.login(token.unwrap()).await;
}
