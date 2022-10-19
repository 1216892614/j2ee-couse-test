use yew::prelude::*;

#[function_component(Home)]
pub(super) fn home() -> Html {
    html! {
        <>
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
