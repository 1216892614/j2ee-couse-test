use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum AppRoute {
    #[at("/")]
    Home,
    #[at("/login/:s")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub(crate) fn app_switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html!(<super::home::Home />),
        AppRoute::Login => html!(<super::login::Login />),
        AppRoute::NotFound => html!(<super::not_found::NotFound />),
    }
}
