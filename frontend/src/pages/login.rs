use yew::prelude::*;
use yew_router::prelude::*;

mod login_in;
mod login_up;

mod login_success;
mod login_error;

mod route;

#[function_component(Login)]
pub(super) fn login() -> Html {
    html! {
            <div
                style="
                box-shadow: 3px 3px 5px rgba(0, 0, 0, 0.445);
                background-color: rgb(19, 19, 19);
                padding: 10px 20px 10px 20px;
                flex-direction: column;
                align-items: center;
                position: absolute;
                border-radius: 5px;
                display: flex;
                width: 400px;
                margin: auto;
                height: 70%;
                bottom: 0;
                right: 0;
                top: 10%;
                left: 0;
            "
            >
                <BrowserRouter>
                    <Switch<route::LoginRoute> render={Switch::render(route::login_switch)} />
                </BrowserRouter>
            </div>
    }
}
