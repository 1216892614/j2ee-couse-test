use gloo::{dialogs::alert, net::http::Request};
use yew::UseStateHandle;

use crate::api::LOGIN_SERVE;

#[derive(Debug, Clone)]
pub(super) struct User {
    pub(super) username: String,
    pub(super) password: String,
}

impl User {
    fn is_password_and_username_valid(&self) -> bool {
        let username_len = self.username.chars().count();
        let password_len = self.password.chars().count();

        let (symbol, uppercase, lowercase) = self
            .password
            .chars()
            .map(|i| {
                (
                    match i {
                        'a'..='z' | 'A'..='Z' => false,
                        _ => true,
                    },
                    match i {
                        'a'..='z' => true,
                        _ => false,
                    },
                    match i {
                        'A'..='Z' => true,
                        _ => false,
                    },
                )
            })
            .fold((false, false, false), |acc, x| {
                (acc.0 || x.0, acc.1 || x.1, acc.2 || x.2)
            });

        username_len > 3
            && username_len < 125
            && password_len > 8
            && password_len < 125
            && symbol
            && uppercase
            && lowercase
    }

    pub(super) fn login_in_request(
        &self,
    ) -> Option<gloo::net::http::Request> {
        if self.is_password_and_username_valid() {
            let username = hex::encode(&self.username);
            let timestamp = js_sys::Date::new_0().get_time() / 1000.0;
            let password = hex::encode(&self.blake2b_psw_mac(timestamp));

            Some(Request::post(&format!(
                "{}/login_in/{}/{}/{}",
                LOGIN_SERVE, username, password, timestamp
            )))
        } else {
            alert("错误的账号或密码");
            None
        }
    }

    pub(super) fn login_up_request(
        &self,
    ) -> Option<gloo::net::http::Request> {
        if self.is_password_and_username_valid() {
            let username = hex::encode(&self.username);
            let timestamp = js_sys::Date::new_0().get_time();
            let password = hex::encode(&self.blake2b_256_psw());

            Some(Request::post(
                &(format!(
                    "{}/login_up/{}/{}/{}",
                    LOGIN_SERVE, username, password, timestamp
                )),
            ))
        } else {
            alert("账号长度应在 3~125 之间\n密码需包含大小写和一个字符, 长度在 8~125 之间");
            None
        }
    }

    fn blake2b_256_psw(&self) -> [u8; 32] {
        use cryptoxide::{blake2b::Blake2b, digest::Digest};

        let mut digest = [0u8; 32];
        let mut context = Blake2b::new(32);
        context.input(self.password.as_bytes());
        context.result(&mut digest);

        digest
    }

    fn blake2b_psw_mac(&self, timestamp: f64) -> Vec<u8> {
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
        context.input(&self.blake2b_256_psw());

        context.result().code().to_ascii_lowercase()
    }
}
