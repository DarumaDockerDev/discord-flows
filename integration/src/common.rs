use lru::LruCache;
use once_cell::sync::OnceCell;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serenity::client::bridge::gateway::ShardManager;
use sqlx::PgPool;
use std::{collections::HashMap, num::NonZeroUsize, sync::Arc};
use tokio::sync::Mutex;

pub fn shard_map() -> &'static Mutex<HashMap<String, Arc<Mutex<ShardManager>>>> {
    static INSTANCE: OnceCell<Mutex<HashMap<String, Arc<Mutex<ShardManager>>>>> = OnceCell::new();
    INSTANCE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn get_client() -> &'static Client {
    static INS: OnceCell<Client> = OnceCell::new();
    INS.get_or_init(Client::new)
}

pub fn get_cache() -> &'static Mutex<LruCache<String, String>> {
    static INS: OnceCell<Mutex<LruCache<String, String>>> = OnceCell::new();
    INS.get_or_init(|| {
        let cache = LruCache::new(NonZeroUsize::new(30).unwrap());
        Mutex::new(cache)
    })
}

pub async fn check_token(token: &str) -> bool {
    let url = "https://discord.com/api/users/@me";

    let client = get_client();
    let resp = client
        .get(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bot {token}"))
        .send()
        .await;

    if let Ok(r) = resp {
        if r.status().is_success() {
            return true;
        }
    }

    false
}

pub async fn del_and_shutdown(
    flow_id: &str,
    flows_user: &str,
    bot_token: &str,
    pool: &PgPool,
) -> Result<StatusCode, String> {
    let sql = "
        DELETE FROM listener
        WHERE flow_id = $1 AND flows_user = $2 AND bot_token = $3
    ";
    sqlx::query(sql)
        .bind(flow_id)
        .bind(flows_user)
        .bind(bot_token)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    let mut guard = shard_map().lock().await;
    let v = guard.remove(bot_token);

    if let Some(shard_manager) = v {
        shard_manager.lock().await.shutdown_all().await;
    }
    drop(guard);

    Ok(StatusCode::OK)
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct Flow {
    pub flows_user: String,
    pub flow_id: String,
}

#[derive(sqlx::FromRow)]
pub struct Bot {
    #[sqlx(rename = "bot_token")]
    pub token: String,
}

#[derive(Deserialize)]
pub struct ListenerQuery {
    pub bot_token: String,
}
