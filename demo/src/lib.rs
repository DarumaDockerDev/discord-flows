use std::env;

use discord_flows::{get_client, listen_to_event, model::Message};
use serde_json::json;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let token = env::var("DISCORD_TOKEN").unwrap();

    listen_to_event(&token, |msg| handle(msg, &token)).await;
}

async fn handle(msg: Message, token: &str) {
    let client = get_client(token);

    let channel_id = msg.channel_id;
    let content = msg.content;

    _ = client
        .send_message(
            channel_id.into(),
            &json!({
                "content": content,
            }),
        )
        .await;
}
