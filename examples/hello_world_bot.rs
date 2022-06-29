use disrust::{Bot, Event, Intent::*};

#[tokio::main]
async fn main() {
    let token = option_env!("TOKEN");
    let mut bot = Bot::new(token.unwrap());
    bot.login(vec![Guild, GuildMessages], |e| match e {
        Event::Ready { .. } => {
            println!("The Bot's ready!");
        }
        Event::GuildCreate(guild) => {
            println!("{:#?}", guild);
        }
        _ => {}
    })
    .await;
}
