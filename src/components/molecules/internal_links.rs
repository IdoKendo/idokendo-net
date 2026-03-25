use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<String>,
}

#[function_component(InternalLinks)]
pub fn internal_links(props: &Props) -> Html {
    let onsubmit = props.onsubmit.clone();
    let navigator = use_navigator().expect("Failed to load navigator");
    let onclick_home_callback = Callback::from(move |_| {
        navigator.push(&Route::Home);
        onsubmit.emit("Home".to_owned());
    });

    let onsubmit = props.onsubmit.clone();
    let navigator = use_navigator().expect("Failed to load navigator");
    let onclick_blog_callback = Callback::from(move |_| {
        navigator.push(&Route::Blog);
        onsubmit.emit("Blog".to_owned());
    });
    html! {
        <nav>
            <ul class="int-links">
                <li>
                    <button onclick={onclick_home_callback}>{"Home"}</button>
                </li>
                <li>
                    <button onclick={onclick_blog_callback}>{"Blog"}</button>
                </li>
            </ul>
        </nav>
    }
}
