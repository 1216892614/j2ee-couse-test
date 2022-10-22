use yew::prelude::*;

use crate::component::infinite_for;

#[function_component(Home)]
pub(super) fn home() -> Html {
    let padding_state = use_state(|| html!(<></>));

    let request = Callback::from(|i: (usize, Callback<Html>)| {
        i.1.emit(html!(
            <div
                style="
                    min-height: 300px;
                    min-width: 700px;
                    height: 300px;
                    width: 700px;
                    border-radius: 5px;
                    background-color: rgba(222, 240, 255, 0.262);
                    margin: 5px;
                "
            >
                <h1 style="color: black;">{i.0}</h1>
            </div>
        ))
    });

    html! {
        <>
                <infinite_for::InfiniteFor {request}/>
        </>
    }
}
