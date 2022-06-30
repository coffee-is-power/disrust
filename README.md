# DisRust
Async, Event-driven library for creating discord bots in rust
**Warning**:
This library is still experimental, the API will change very often without any notice through the development of this crate.
# Example
```rust
use disrust::{Bot, Event, Intent::*};

#[tokio::main]
async fn main() {
    let mut bot = Bot::new("Token Here");
    // Login into your bot
    // You need ot specify your intents here and a event handler function
    // Which will be called on every event the library receives from the discord gateway
    bot.login(vec![Guild, GuildMessages, MessageContent], |e| async {
        match e {
            // Message create is called when a message is sent
            Event::MessageCreate(msg) => {
                if msg.content() == "!hello"{
                    msg.channel().await.send_message("Hello!".to_string()).await.unwrap();
                }
            }
        // Ignore other events
        _ => {}
    }})
    .await;
}
```
