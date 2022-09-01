use std::ops::Deref;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use super::user_data::User;

#[function_component(LoginIn)]
pub(super) fn login_in() -> Html {
    let user_state = use_state(|| User {
        username: String::new(),
        password: String::new(),
    });

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

    let login_state = use_state(|| String::new());

    let submit_onclick = Callback::from(move |_| {
        let login_state = login_state.clone();

        user_state.deref().login_in_request(login_state);
    });

    html! {
            <>
                <div style="flex-direction: column">
                    <h1>{"登录"}</h1>
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

                <div>
                    <button onclick={submit_onclick}>{"登录"}</button>
                </div>
                <div style="flex-shrink: 1"></div>

                <div style="flex-direction: column">
                    <h4>{"还没有账号?"}</h4>
                    <div>
                        <Link<super::route::LoginRoute>
                            to={super::route::LoginRoute::LoginUp}
                        >
                            <button>{"注册"}</button>
                        </Link<super::route::LoginRoute>>
                    </div>
                </div>
            </>
    }
}
