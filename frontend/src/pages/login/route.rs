use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum LoginRoute {
    #[at("/login/login_success")]
    LoginSuccess,
    #[at("/login/login_up")]
    LoginUp,
    #[not_found]
    #[at("/login/login_in")]
    LoginIn,
}

pub(crate) fn login_switch(routes: &LoginRoute) -> Html {
    match routes {
        LoginRoute::LoginSuccess => html!(<super::login_success::LoginSuccess />),
        LoginRoute::LoginUp => html!(<super::login_up::LoginUp />),
        LoginRoute::LoginIn => html!(<super::login_in::LoginIn />),
    }
}
