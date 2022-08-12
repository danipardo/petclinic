use rand::{distributions::Alphanumeric, Rng};
use rbatis::{crud::CRUD, crud_table, rbatis::Rbatis};
use redis::{ErrorKind, FromRedisValue};

use sha1::{Digest, Sha1};
use std::collections::HashMap;

#[crud_table]
#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password: String,
}

impl FromRedisValue for User {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        if let redis::Value::Data(u) = v {
            let s = String::from_utf8_lossy(&*u);
            let user: User = serde_json::from_str(&s).unwrap();
            return Ok(user);
        }

       Err((ErrorKind::TypeError, "Parse to JSON Failed").into())
    }
}

pub async fn authenticate(
    rb: &Rbatis,
    login: &str,
    password: &str,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();

    map.insert("username", login);

    let w = rb.new_wrapper().eq("username", login);
    let mut users: Vec<User> = rb.fetch_list_by_wrapper(w).await.unwrap();

    if users.len() == 1 {
        let user = users.pop().unwrap();
        // Password verification
        let mut hasher = Sha1::new();
        hasher.update(password);
        let encrypted: String = format!("{:x}", hasher.finalize());
        if encrypted == user.password || user.password.is_empty() {
            return Ok(Some(user));
        } else {
            tracing::error!("Wrong password for user: {}", login);
            return Ok(None);
        }
    } else {
        tracing::error!("User not found: {}", login);
    }
    Ok(None)
}

pub fn session_key() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    s
}
