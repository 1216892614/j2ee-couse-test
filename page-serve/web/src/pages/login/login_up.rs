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
                        alert("????????????");
                        history.push(route::LoginRoute::LoginIn);
                    }
                    "USERNAME_ALREADY_EXISTS" => alert("??????????????????"),
                    "SERVER_SIDE_ERROR" => alert("?????????????????????"),
                    _ => alert("???????????????"),
                }
            })
        } else {
            alert("???????????????????????????!");
        }
    });

    html! {
            <>
                <div style="flex-direction: column">
                    <h1>{"??????"}</h1>
                </div>

                <hr />

                <div style="height: 10%"></div>

                <div style="flex-direction: column">
                    <h3>{"?????????"}</h3>
                    <input placeholder="?????? / ?????? / ?????????" onchange={username_onchange}/>
                </div>

                <div style="flex-direction: column">
                    <h3>{"??????"}</h3>
                    <input placeholder="????????????" type="password" onchange={password_onchange}/>
                </div>

                <div style="flex-direction: column">
                    <h3>{"????????????"}</h3>
                    <input placeholder="????????????" type="password" onchange={repeat_password_onchange}/>
                </div>

                <div>
                    <button onclick={submit_onclick}>{"??????"}</button>
                </div>
                <div style="flex-shrink: 1"></div>

                <div style="flex-direction: column">
                    <h4>{"???????????????????"}</h4>
                    <div>
                        <Link<super::route::LoginRoute>
                            to={super::route::LoginRoute::LoginIn}
                        >
                            <button>{"??????"}</button>
                        </Link<super::route::LoginRoute>>
                    </div>
                </div>
            </>
    }
}
