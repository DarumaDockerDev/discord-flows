use std::sync::Arc;

use axum::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};
use sqlx::PgPool;

use crate::model::Gid;
use crate::shared::get_client;
use crate::{DEFAULT_BOT_PLACEHOLDER, HOOK_URL};

pub struct Handler {
    pub token: String,
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        if self.token == DEFAULT_BOT_PLACEHOLDER {
            let guild_id = msg.guild_id;
            let channel_id = msg.channel_id;
            if match guild_id {
                Some(gid) => {
                    let select = "
                        SELECT guild_id
                        FROM filter
                        WHERE guild_id = $1 AND channel_id = $2
                    ";
                    let gid: Result<Gid, _> = sqlx::query_as(select)
                        .bind(gid.as_u64().to_string())
                        .bind(channel_id.as_u64().to_string())
                        .fetch_one(&*self.pool)
                        .await;

                    gid.is_err()
                }
                None => false,
            } {
                return;
            }
        }

        let client = get_client();
        _ = client
            .post(HOOK_URL.as_str())
            .json(&msg)
            .header("X-Discord-token", &self.token)
            .send()
            .await;
    }
}
