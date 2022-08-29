use yew::prelude::*;

#[function_component(LoginSuccess)]
pub(super) fn login_success() -> Html {
    html! {
            <>
                <div style="flex-direction: column">
                    <h1>{"登录成功"}</h1>
                </div>
                <div style="flex-shrink: 1"></div>
                <div>
                    <div>
                        <button>{"退出登录"}</button>
                    </div>
                </div>
            </>
    }
}