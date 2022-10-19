use std::ops::Deref;

use gloo::dialogs::alert;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use super::{route, user_data::User};

#[function_component(LoginUp)]
pub(super) fn login_up() -> Html {
    let user_state = use_state(|| User {
        username: String::new(),
        password: String::new(),
    });

    let repeat_password_state = use_state(|| String::new());

    let username_onchange = {
        let user_state = user_state.clone();
        Callback::from(move |e: Event| {
            let username = e
                .target()
                .expect("cannot get username from Input in login_in.rs!")
                .unchecked_into::<HtmlInputElement>()
                .value();

            let password = user_state.deref().password.clone();

            user_state.set(User { username, password })
        })
    };

    let password_onchange = {
        let user_state = user_state.clone();
        Callback::from(move |e: Event| {
            let password = e
                .target()
                .expect("cannot get password from Input in login_in.rs!")
                .unchecked_into::<HtmlInputElement>()
                .value();

            let username = user_state.deref().username.clone();

            user_state.set(User { username, password })
        })
    };

    let repeat_password_onchange = {
        let repeat_password_state = repeat_password_state.clone();
        Callback::from(move |e: Event| {
            let repeat_password = e
                .target()
                .expect("cannot get password from Input in login_in.rs!")
                .unchecked_into::<HtmlInputElement>()
                .value();

            repeat_password_state.set(repeat_password)
        })
    };

    let history = use_history().unwrap();

    let submit_onclick = Callback::from(move |_| {
        let history = history.clone();

        let repeat_password_state = repeat_password_state.clone();

        if *repeat_password_state == user_state.deref().password {
            let request = match user_state.deref().login_up_request() {
                Some(r) => r,
                None => {
                    return;
                }
            };

            wasm_bindgen_futures::spawn_local(async move {
                let login_state = request.send().await.unwrap().text().await.unwrap();

                match &login_state as &str {
                    "SCSS" => {
                        alert("注册成功");
                        history.push(route::LoginRoute::LoginIn);
                    }
                    "USERNAME_ALREADY_EXISTS" => alert("用户名已存在"),
                    "SERVER_SIDE_ERROR" => alert("服务器发生错误"),
                    _ => alert("请刷新重试"),
                }
            })
        } else {
            alert("两次密码输入不一样!");
        }
    });

    html! {
            <>
                <div style="flex-direction: column">
                    <h1>{"注册"}</h1>
                </div>

                <hr />

                <div style="height: 10%"></div>

                <div style="flex-direction: column">
                    <h3>{"用户名"}</h3>
                    <input placeholder="邮箱 / 手机 / 用户名" onchange={username_onchange}/>
                </div>

                <div style="flex-direction: column">
                    <h3>{"密码"}</h3>
                    <input placeholder="输入密码" type="password" onchange={password_onchange}/>
                </div>

                <div style="flex-direction: column">
                    <h3>{"重复密码"}</h3>
                    <input placeholder="输入密码" type="password" onchange={repeat_password_onchange}/>
                </div>

                <div>
                    <button onclick={submit_onclick}>{"注册"}</button>
                </div>
                <div style="flex-shrink: 1"></div>

                <div style="flex-direction: column">
                    <h4>{"已经有账号了?"}</h4>
                    <div>
                        <Link<super::route::LoginRoute>
                            to={super::route::LoginRoute::LoginIn}
                        >
                            <button>{"登录"}</button>
                        </Link<super::route::LoginRoute>>
                    </div>
                </div>
            </>
    }
}
