use yew::prelude::*;
use yew_router::prelude::*;

mod user_data;

mod login_in;
mod login_up;

mod login_success;

mod route;

#[function_component(Login)]
pub(super) fn login() -> Html {
    html! {
            <div
                style="
                box-shadow: 3px 3px 5px rgba(0, 0, 0, 0.445);
                background-color: rgb(19, 19, 19);
                align-items: center;
                position: absolute;
                display: flex;
                flex-direction: column;
                border-radius: 5px;
                margin: auto;
                padding: 10px 20px 10px 20px;
                width: 400px;
                height: 70%;
                top: 10%;
                bottom: 0;
                left: 0;
                right: 0;
            "
            >
                <BrowserRouter>
                    <Switch<route::LoginRoute> render={Switch::render(route::login_switch)} />
                </BrowserRouter>
            </div>
    }
}
