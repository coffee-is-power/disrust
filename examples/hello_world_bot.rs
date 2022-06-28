use disrust::{Bot, Event, Intent::*};

#[tokio::main]
async fn main() {
    let mut bot = Bot::new();
    let token = option_env!("TOKEN");
    bot.login(token.unwrap(),vec![Guild, GuildMessages],|e| match e {
        Event::Ready { .. } => {
            println!("The Bot's ready!");
        }
        Event::GuildCreate(guild) => {
            println!("{:#?}", guild);
        }
        _ => {}
    }).await;
}
