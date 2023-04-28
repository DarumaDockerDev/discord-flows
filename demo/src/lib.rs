use std::env;

use discord_flows::get_client;
use slack_flows::{listen_to_channel, SlackMessage};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let token = env::var("DISCORD_TOKEN").unwrap();

    let team_name = env::var("team").unwrap_or("ham-5b68442".to_string());
    let channel_name = env::var("channel").unwrap_or("general".to_string());

    // listen_to_event(token.clone(), move |msg| handle(msg, token)).await;
    listen_to_channel(&team_name, &channel_name, |msg| cb(msg, &token));
}

fn cb(msg: SlackMessage, token: &str) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(handle(msg, token))
}

async fn handle(msg: SlackMessage, token: &str) {
    let client = get_client(token);

    let channel_id = 1097913977058627707;
    let content = msg.text;

    _ = client
        .send_message(
            channel_id,
            &serde_json::json!({
                "content": content,
            }),
        )
        .await;
}
