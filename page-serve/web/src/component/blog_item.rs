use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub(crate) struct Props {
    pub(crate) item_num: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Blog {
    username: String,
    published_time: String,
    body: String,
}

#[function_component(BlogItem)]
pub(crate) fn blog_item(props: &Props) -> Html {
    let blog_state = use_state(|| -> Option<Blog> { None });

    {
        let blog_state = blog_state.clone();
        let item_num = props.item_num;

        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                gloo::timers::callback::Timeout::new(3000, move || {
                    blog_state.set(Some(Blog {
                        username: "123".to_owned(),
                        body: item_num.to_string(),
                        published_time: "2022/2/22".to_owned(),
                    }));
                })
                .forget();
            });
            || ()
        })
    }

    if let Some(blog) = (*blog_state).clone() {
        html!(
            <div
                style="\
                    max-height: 600px;\
                    max-width: 700px;\
                    min-height: 300px;\
                    min-width: 700px;\
                    height: auto;\
                    width: auto;\
                    flex-direction: column;\
                    align-items: center;\
                    border-radius: 5px;\
                    background-color: rgb(30, 34, 39);\
                    margin: 5px;\
                "
            >
                <div>
                    <span
                        readonly={true}
                        style="
                            min-height: 255px;\
                            min-width: 660px;\
                            border-radius: 5px;\
                            word-wrap: break-word;\
                            word-break: break-all;\
                            overflow: auto;\
                            background-color: rgb(35, 39, 46);\
                            scrollbar-color: rgb(35, 39, 46);\
                            scrollbar-width: thin;\
                            width: 100%;\
                            border: none;\
                            padding: 10px 20px 0 20px;\
                            max-height: 255px;\
                            max-width: 660px;\
                        "
                    >
                         {blog.body}
                    </span>
                </div>
                <div
                    style="
                        height: 30px;\
                        margin: 5px;\
                        justify-content: end;\
                        padding: 5px 5px 0 5px;\
                    "
                >
                    <a style="margin-left: 10px">{blog.username}</a>
                    <a style="margin-left: 10px; margin-right: 20px">
                        {blog.published_time}
                    </a>
                </div>
            </div>
        )
    } else {
        html!(
            <div
                style="
                    min-height: 300px;\
                    min-width: 700px;\
                    height: 300px;\
                    width: 700px;\
                    border-radius: 5px;\
                    background-color: rgba(222, 240, 255, 0.262);\
                    margin: 5px;\
                "
            ></div>
        )
    }
}
