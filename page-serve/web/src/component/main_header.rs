use crate::pages::route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(MainHeader)]
pub(crate) fn main_header() -> Html {
    html! {
        <>
            <div class="top">
                <center>
                    <ul>
                        <li>
                            <Link<route::AppRoute>
                                to={route::AppRoute::Home}
                            >
                                { "J2EE Course Test" }
                            </Link<route::AppRoute>>
                        </li>
                        <li>
                            <Link<route::AppRoute>
                                to={route::AppRoute::NotFound}
                            >
                                { "NotFound" }
                            </Link<route::AppRoute>>
                        </li>
                        <li>
                            <Link<route::AppRoute>
                                to={route::AppRoute::Login}
                            >
                                { "登录" }
                            </Link<route::AppRoute>>
                        </li>
                    </ul>
                </center>
            </div>
        </>
    }
}
