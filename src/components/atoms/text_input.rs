use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
    pub onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let handle_onchange = props.onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .expect("Failed to get event target")
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(value);
    });
    html! {
        <input type="text" name={props.name.clone()} onchange={onchange} placeholder={props.name.clone()} />
    }
}
