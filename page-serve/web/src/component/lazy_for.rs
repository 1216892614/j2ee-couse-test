use web_sys::HtmlDivElement;
use yew::{html::ChildrenRenderer, prelude::*};

#[derive(Debug, Clone, PartialEq, Properties)]
pub(crate) struct Props {
    padding: Option<Children>,
    // request: Callback<Callback<Html>>,
}

#[function_component(LazyFor)]
pub(crate) fn lazy_for(props: &Props) -> Html {
    let padding = props.padding.clone();

    let list_state = use_state(|| html!(<></>));

    let limit_ref = NodeRef::default();
    let list_ref = NodeRef::default();

    let onscroll = {
        let limit_ref = limit_ref.clone();
        let list_ref = list_ref.clone();

        Callback::from(move |_| {
            let limit_ref = limit_ref.cast::<HtmlDivElement>().unwrap();
            let limit_pos = (limit_ref.offset_left(), limit_ref.scroll_top());

            let list_ref = list_ref.cast::<HtmlDivElement>().unwrap();
            let list_pos = (list_ref.offset_left(), list_ref.scroll_top());

            gloo::console::log!(format!("limit: {:?} \n list: {:?}", limit_pos, list_pos));
        })
    };

    html! {
        <>
            <div
                ref={list_ref}
                {onscroll}
                style="
                    height: 100%;
                    width: 100%;
                    margin-bottom: 5px;
                    display: flex;
                    justify-content: flex-start;
                    align-items: center;
                    flex-direction: column;
                    overflow: auto;
                "
            >
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <h1>
                    {"1234"}
                </h1>
                <div
                    ref={limit_ref}
                    style="
                        height:1px;
                        max-width:0;
                        padding:0;
                        border:0;
                        margin:0;
                    "
                ></div>
                {
                    if let Some(padding) = padding {
                        padding
                    } else {
                        ChildrenRenderer::new(
                            vec![html!(
                                <>
                                    <h4
                                        style="
                                            background: rgb(18,23,46);
                                            padding: 5px;
                                            border-radius: 5px;
                                        "
                                    >
                                        {"加载中..."}
                                    </h4>
                                </>
                            )]
                        )
                    }
                }
            </div>
        </>
    }
}
