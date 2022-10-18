use std::time::{self, UNIX_EPOCH};

use crate::static_setting::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub(super) async fn login_in(username: String, password: String, timestamp: f64) -> String {
    let now_timestamp = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64();

    if now_timestamp - timestamp >= ACCEPT_TIME_DIFFERENCE && now_timestamp >= timestamp {
        return "login_serve::login_in::JWT Time Out".to_owned();
    }

    if let Ok(token) = encode(
        &Header::default(),
        &Claims::new(now_timestamp, TOKEN_LIVE_LIFE),
        &EncodingKey::from_secret(todo!()),
    ) {
        token
    } else {
        "login_serve::login_in::Unable to issue JWT".to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: f64,
}

impl Claims {
    fn new(now_timestamp: f64, life: f64) -> Self {
        Self {
            exp: now_timestamp + life,
        }
    }
}
