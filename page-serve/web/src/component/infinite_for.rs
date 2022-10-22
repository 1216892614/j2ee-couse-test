use web_sys::HtmlDivElement;
use yew::prelude::*;

static mut RENDER_LIST: Option<Html> = None;

#[derive(Debug, Clone, PartialEq, Properties)]
pub(crate) struct Props {
    pub(crate) request: Option<Callback<(usize, Callback<Html>)>>,
    pub(crate) loading_block: Option<Children>,
    pub(crate) is_flex_direction_row: Option<bool>,
}

#[function_component(InfiniteFor)]
pub(crate) fn infinite_for(props: &Props) -> Html {
    let request = props
        .request
        .clone()
        .unwrap_or(Callback::from(|i: (usize, Callback<Html>)| {
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
                    <h1>{i.0}</h1>
                </div>
            ))
        }));

    let loading_block = props.loading_block.clone();
    let request_number_state = use_state(|| 0usize);
    let request_times_state = use_state(|| 0usize);
    let list_state = use_state(|| html!(<></>));
    let is_bottom_state = use_state(|| true);

    let is_flex_direction_row = props.is_flex_direction_row;
    let list_ref = NodeRef::default();
    let loading_block_ref = NodeRef::default();

    let list_pust = {
        Callback::from(move |item: Html| unsafe {
            RENDER_LIST = Some(
                [RENDER_LIST.clone().unwrap_or(html!(<></>)), item]
                    .into_iter()
                    .collect(),
            )
        })
    };

    {
        let list_ref = list_ref.clone();
        let request = request.clone();
        let list_pust = list_pust.clone();
        let list_state = list_state.clone();
        let request_number_state = request_number_state.clone();
        let request_times_state = request_times_state.clone();

        use_effect(move || {
            let list_ref = list_ref.cast::<HtmlDivElement>().unwrap();

            if list_ref.scroll_height() - 100 < list_ref.client_height()
                && list_ref.scroll_width() - 100 < list_ref.client_width()
            {
                request_number_state.set(*request_number_state + 1);
                request_times_state.set(*request_times_state + 1);

                let key = *request_times_state;

                request.emit((key, list_pust));
                unsafe { list_state.set(RENDER_LIST.clone().unwrap_or(html!(<></>))) }
            }
            || ()
        })
    };

    let onscroll = {
        let list_ref = list_ref.clone();
        let loading_block_ref = loading_block_ref.clone();
        let is_bottom_state = is_bottom_state.clone();

        let list_pust = list_pust.clone();
        let request = request.clone();
        let list_state = list_state.clone();
        let request_number_state = request_number_state.clone();
        let request_times_state = request_times_state.clone();

        Callback::from(move |_| {
            let list_ref = list_ref.cast::<HtmlDivElement>().unwrap();
            let list_pos = (list_ref.scroll_left(), list_ref.scroll_top());

            let refresh_range = loading_block_ref
                .cast::<HtmlDivElement>()
                .unwrap()
                .client_height();

            let befor_is_bottom_state = *is_bottom_state;

            let after_is_bottom_state = match (
                is_flex_direction_row,
                list_ref.scroll_width() - list_pos.0 - list_ref.client_width() <= refresh_range,
                list_ref.scroll_height() - list_pos.1 - list_ref.client_height() <= refresh_range,
            ) {
                (Some(true), true, _) | (None, _, true) | (Some(false), _, true) => true,
                _ => false,
            };

            if !befor_is_bottom_state && after_is_bottom_state {
                request_times_state.set(*request_times_state + *request_number_state);

                for i in 0..*request_number_state {
                    let key = *request_times_state + i;

                    request.emit((key, list_pust.clone()));
                    unsafe { list_state.set(RENDER_LIST.clone().unwrap_or(html!(<></>))) }
                }
            }

            list_ref.scroll_to_with_x_and_y(list_pos.0 as f64, list_pos.1 as f64);

            is_bottom_state.set(after_is_bottom_state);
        })
    };

    html! {
        <>
            <div
                ref={list_ref}
                {onscroll}
                style={
                    format!("
                        height: 100%;
                        width: 100%;
                        margin-bottom: 5px;
                        display: flex;
                        justify-content: flex-start;
                        align-items: center;
                        flex-direction: {};
                        overflow: auto;
                    ",
                    if is_flex_direction_row == Some(true) {"row"}
                    else {"column"})
                }
            >
                //======List=======
                {(*list_state).clone()}
                //==Loading Block==
                <div
                    ref={loading_block_ref}
                    style="
                        width: auto;
                        height: auto;
                    "
                >
                    {
                        if let Some(loading_block) = loading_block {
                            loading_block
                        } else {
                            Children::new(vec![html!(
                                <h4
                                    style="
                                        background: rgb(18,23,46);
                                        padding: 5px;
                                        border-radius: 5px;
                                    "
                                >
                                    {"到底了"}
                                </h4>
                            )])
                        }
                    }
                </div>
            </div>
        </>
    }
}
