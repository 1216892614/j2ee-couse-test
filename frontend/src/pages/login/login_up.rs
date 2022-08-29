use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(LoginUp)]
pub(super) fn login_up() -> Html {
    html! {
            <>
                <div style="flex-direction: column">
                    <h1>{"注册"}</h1>
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

                <div style="flex-direction: column">
                    <h3>{"重复密码"}</h3>
                    <input placeholder="输入密码" type="password"/>
                </div>
                <div>
                    <div>
                        <button>{"登录"}</button>
                    </div>
                </div>
                <div style="flex-shrink: 1"></div>

                <div style="flex-direction: column">
                    <h4>{"已经有账号了?"}</h4>
                    <div>
                        <Link<super::route::LoginRoute>
                            to={super::route::LoginRoute::LoginIn}
                        >
                            <button>{"登录"}</button>
                        </Link<super::route::LoginRoute>>
                    </div>
                </div>
            </>
    }
}
