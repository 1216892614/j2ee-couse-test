use crate::pages::route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(MainHeader)]
pub(crate) fn main_header() -> Html {
    html! {
        <>
            <div
                style="
                    background: rgb(19, 19, 19);
                    min-height: 30px; 
                    height: 30px; 
                    width: 100%;
                    margin-bottom: 5px;
                    display: flex;
                    justify-content: flex-start;
                    align-items: center;
                    flex-direction: row;
                "
            >
                        <a
                            class="top"
                        >
                            <Link<route::AppRoute>
                                to={route::AppRoute::Home}
                            >
                                { "J2EE Course Test" }
                            </Link<route::AppRoute>>
                        </a>
                        <a
                            class="top"
                        >
                            <Link<route::AppRoute>
                                to={route::AppRoute::NotFound}
                            >
                                { "NotFound" }
                            </Link<route::AppRoute>>
                        </a>
                        <a
                            class="top"
                        >
                            <Link<route::AppRoute>
                                to={route::AppRoute::Login}
                            >
                                { "登录" }
                            </Link<route::AppRoute>>
                        </a>
            </div>
        </>
    }
}
