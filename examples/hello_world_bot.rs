#[tokio::main]
async fn main(){
    let token = option_env!("TOKEN");
    let mut bot = disrust::Bot::new(token.expect("No token was provided at compile time"), disrust::GatewayIntents::GUILDS as i32).await;
    bot.login().await;
    
}
