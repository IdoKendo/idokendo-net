use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub onclick: Option<Callback<Option<String>>>,
    pub message: Option<String>,
}

#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    let message = props.message.clone();
    if let Some(onclick) = onclick {
        let button_onclick = Callback::from(move |_| {
            onclick.emit(message.clone());
        });
        return html! {
            <button onclick={button_onclick}>{props.label.clone()}</button>
        };
    };
    html! {
        <button>{props.label.clone()}</button>
    }
}
