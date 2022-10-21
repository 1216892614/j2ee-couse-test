use yew::prelude::*;

use crate::component::lazy_for;

#[function_component(Home)]
pub(super) fn home() -> Html {
    html! {
        <>
                <lazy_for::LazyFor />
        </>
    }
}
