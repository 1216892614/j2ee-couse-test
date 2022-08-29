use yew::prelude::*;

#[function_component(NotFound)]
pub(super) fn not_found() -> Html {
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
                >{ "NotFound" }</h1>
            </center>
        </>
    }
}
