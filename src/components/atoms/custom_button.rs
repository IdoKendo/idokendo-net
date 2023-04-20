use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub onclick: Option<Callback<()>>,
}

#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    if let Some(onclick) = onclick {
        let button_onclick = Callback::from(move |_| {
            onclick.emit(());
        });
        return html! {
            <button onclick={button_onclick}>{props.label.clone()}</button>
        };
    };
    html! {
        <button>{props.label.clone()}</button>
    }
}
