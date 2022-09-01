use gloo::{console::log, dialogs::alert, net::http::Request};
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

    pub(super) fn login_in_request(&self, login_state: UseStateHandle<String>) {
        if self.is_password_and_username_valid() {
            let username = self.username.clone();
            let password = self.password.clone();
            let timestamp = js_sys::Date::new_0().get_time().to_string();

            wasm_bindgen_futures::spawn_local(async move {
                let fetched_login_result: String = Request::get(
                    &(format!(
                        "{}/login_in/{}/{}/{}",
                        LOGIN_SERVE, username, password, timestamp
                    )),
                )
                .send()
                .await
                .unwrap()
                .json()
                .await
                .expect("cannot deserialization ans from .../login/request/login_in/ request");

                log!(&fetched_login_result);

                login_state.set(fetched_login_result);
            })
        } else {
            alert("错误的账号或密码")
        }
    }

    pub(super) fn login_up_request(&self, login_state: UseStateHandle<String>) {
        if self.is_password_and_username_valid() {
            let username = self.username.clone();
            let password = self.password.clone();
            let timestamp = js_sys::Date::new_0().get_time().to_string();

            wasm_bindgen_futures::spawn_local(async move {
                let fetched_login_result: String = Request::get(
                    &(format!(
                        "{}/login_up/{}/{}/{}",
                        LOGIN_SERVE, username, password, timestamp
                    )),
                )
                .send()
                .await
                .unwrap()
                .json()
                .await
                .expect("cannot deserialization ans from .../login/request/login_up/ request");

                login_state.set(fetched_login_result);
            })
        } else {
            alert("账号长度应在 3~125 之间\n密码需包含大小写和一个字符, 长度在 8~125 之间")
        }
    }
}
