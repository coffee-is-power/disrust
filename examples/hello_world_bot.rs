use disrust::{Bot, Intent::*, Event};

#[tokio::main]
async fn main() {
    let mut bot = Bot::new(vec![Guild, GuildMessages]).await;
    let token = option_env!("TOKEN");
    bot.set_event_handler(|e| {
        match e {
            Event::Ready { .. } => {
                println!("The Bot's ready!");
            },
            _ => {}
        }
    });
    bot.login(token.unwrap()).await;
}
