use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub(super) fn home() -> Html {
    html! {
        <>
            <div class="top">
                <center>
                    <ul>
                        <li>
                            <Link<super::route::AppRoute>
                                to={super::route::AppRoute::Home}
                            >
                                { "J2EE Course Test" }
                            </Link<super::route::AppRoute>>
                        </li>
                        <li>
                            <Link<super::route::AppRoute>
                                to={super::route::AppRoute::NotFound}
                            >
                                { "NotFound" }
                            </Link<super::route::AppRoute>>
                        </li>
                        <li>
                            <Link<super::route::AppRoute>
                                to={super::route::AppRoute::Login}
                            >
                                { "登录" }
                            </Link<super::route::AppRoute>>
                        </li>
                    </ul>
                </center>
            </div>

            <center>
                <h1
                    style="
                        font-size: 50px;
                        color: rgb(19, 19, 19);
                        align-items: center;
                        margin: auto;
                        position: absolute;
                        bottom: 0;
                        right: 0;
                        top: 10%;
                        left: 0;
                    "
                >{ "HOME" }</h1>
            </center>
        </>
    }
}
