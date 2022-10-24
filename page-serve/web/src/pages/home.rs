use yew::prelude::*;
use yew_infinite_for::InfiniteFor;

use crate::component::blog_item::BlogItem;

#[function_component(Home)]
pub(super) fn home() -> Html {
    let padding_state = use_state(|| html!(<></>));

    let request = Callback::from(|i: (usize, Callback<Html>)| {
        i.1.emit(html!(
            <BlogItem item_num={i.0}/>
        ))
    });

    html! {
        <>
                <InfiniteFor {request}/>
        </>
    }
}
