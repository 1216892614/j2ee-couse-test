use jsonwebtoken::{encode, EncodingKey, Header};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{self, UNIX_EPOCH};

use crate::db::user;
use crate::static_setting::*;

pub(super) async fn login_in(username: String, password: String, timestamp: f64) -> String {
    let now_timestamp = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64();

    if now_timestamp - timestamp >= ACCEPT_TIME_DIFFERENCE || now_timestamp <= timestamp {
        log::debug!(
            "JWT Time Out request: {}, now_time: {}",
            timestamp,
            now_timestamp
        );
        return "REQUEST_TIME_OUT".to_owned();
    }

    let user = match user::UserCli::new() {
        Ok(u) => u,
        Err(e) => {
            log::error!("{e}");
            return SERVER_SIDE_ERROR.to_owned();
        }
    };

    let password_from_db = match user.get_password(username.clone()).await {
        Ok(p) => p,
        Err(e) => {
            log::trace!("{e}");
            return "INCORRECT_USERNAME_OR_PASSWORD".to_owned();
        }
    };

    let password_from_db = match hex::decode(password_from_db) {
        Ok(p) => p,
        Err(e) => {
            log::error!("{e}");
            return SERVER_SIDE_ERROR.to_owned();
        }
    };

    let password = match hex::decode(password) {
        Ok(p) => p,
        Err(e) => {
            log::debug!("{e}");
            return CLI_SIDE_ERROR.to_owned();
        }
    };

    if blake2b_psw_mac(&password_from_db, timestamp) != password {
        log::warn!(
            "INCORRECT_PASSWORD from_db: {:?} != from_request {:?}",
            blake2b_psw_mac(&password_from_db, timestamp),
            password
        );
        return "INCORRECT_USERNAME_OR_PASSWORD".to_owned();
    }

    let secret: String = rand::thread_rng()
        .sample_iter::<char, _>(rand::distributions::Standard)
        .take(SECRET_LEN)
        .collect();

    match user.set_active_token_secret(username, secret.clone()).await {
        Ok(_) => (),
        Err(e) => {
            log::error!("{e}");
            return SERVER_SIDE_ERROR.to_owned();
        }
    };

    match encode(
        &Header::default(),
        &Claims::new(now_timestamp, TOKEN_LIVE_LIFE),
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(token) => token,
        Err(e) => {
            log::error!("{e}");
            SERVER_SIDE_ERROR.to_owned()
        }
    }
}

fn blake2b_psw_mac(psw: &[u8], timestamp: f64) -> Vec<u8> {
    use cryptoxide::{blake2b::Blake2b, mac::Mac};

    let mut key: [u8; 16] = [3, 4, 10, 11, 12, 13, 14, 0, 1, 2, 15, 5, 6, 7, 8, 9];
    for (n, i) in timestamp.to_string().chars().enumerate() {
        if n < 16 {
            if let Ok(i) = i.to_string().parse::<u8>() {
                key[15 - n] = i;
            } else {
                break;
            }
        }
    }

    let mut context = Blake2b::new_keyed(28, &key);
    context.input(psw);

    context.result().code().to_ascii_lowercase()
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
