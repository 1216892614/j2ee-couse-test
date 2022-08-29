use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(LoginIn)]
pub(super) fn login_in() -> Html {
    html! {
            <>
                <div style="flex-direction: column">
                    <h1>{"登录"}</h1>
                </div>
                <hr />
                <div style="height: 10%"></div>

                <div style="flex-direction: column">
                    <h3>{"用户名"}</h3>

                    <input placeholder="邮箱 / 手机 / 用户名" />
                </div>

                <div style="flex-direction: column">
                    <h3>{"密码"}</h3>
                    <input placeholder="输入密码" type="password"/>
                </div>
                <div>
                    <div>
                        <button>{"登录"}</button>
                    </div>
                </div>
                <div style="flex-shrink: 1"></div>

                <div style="flex-direction: column">
                    <h4>{"还没有账号?"}</h4>
                    <div>
                        <Link<super::route::LoginRoute>
                            to={super::route::LoginRoute::LoginUp}
                        >
                            <button>{"注册"}</button>
                        </Link<super::route::LoginRoute>>
                    </div>
                </div>
            </>
    }
}
