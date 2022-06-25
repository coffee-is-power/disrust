use disrust::events::ReadyEvent;

#[tokio::main]
async fn main() {
    let token = option_env!("TOKEN");
    let mut bot = disrust::Bot::new(
        token.expect("No token was provided at compile time"),
        disrust::GatewayIntents::Guilds as i32,
    )
    .await;
    bot.on("ready", |x| {
        println!("Ready: {:#?}", x.as_ref().downcast_ref::<ReadyEvent>().unwrap());
    });
    bot.login().await;
}
