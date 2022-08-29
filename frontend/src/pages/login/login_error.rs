use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(LoginError)]
pub(super) fn login_error() -> Html {
    html! {
            <>
                <div style="flex-direction: column">
                    <h1>{"登录错误"}</h1>
                </div>
                <div style="flex-shrink: 1"></div>
                <div>
                    <div>
                        <Link<super::route::LoginRoute>
                            to={super::route::LoginRoute::LoginIn}
                        >
                            <button>{"重新登录"}</button>
                        </Link<super::route::LoginRoute>>
                    </div>
                </div>
            </>
    }
}
