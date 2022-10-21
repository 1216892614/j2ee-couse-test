use yew::prelude::*;
use yew_router::prelude::*;

pub(crate) mod component;
pub(crate) mod api;
mod pages;
use pages::*;

use crate::component::main_header;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        
            <BrowserRouter>
                <main_header::MainHeader/>
                <Switch<route::AppRoute> render={Switch::render(route::app_switch)} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
