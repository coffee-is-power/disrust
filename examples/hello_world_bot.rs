use disrust::{Bot, Event, Intent::*};

#[tokio::main]
async fn main() {
    let token = option_env!("TOKEN");
    let mut bot = Bot::new(token.unwrap());
    bot.login(vec![Guild, GuildMessages, MessageContent], |e| async {match e {
        Event::MessageCreate(msg) => {
            if msg.content() == "!hello"{
                msg.channel().await.send_message("Hello!".to_string()).await.unwrap();
            }
        }
        _ => {}
    }})
    .await;
}
