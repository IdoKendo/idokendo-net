use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::HeaderText;

#[derive(PartialEq)]
pub enum HeaderStyle {
    Normal,
    Ok,
    Error,
}

impl HeaderStyle {
    pub fn to_string(&self) -> String {
        match self {
            HeaderStyle::Normal => "normal".to_owned(),
            HeaderStyle::Ok => "ok".to_owned(),
            HeaderStyle::Error => "error".to_owned(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub header_style: HeaderStyle,
    pub header_text: HeaderText,
}

#[styled_component(Header)]
pub fn header(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
        .normal {
            color: #768390;
            margin: auto;
            max-width: 750px;
        }
        .ok {
            color: green;
            margin: auto;
            max-width: 750px;
        }
        .error {
            color: red;
            margin: auto;
            max-width: 750px;
        }
    "#
    )
    .expect("Failed to create stylesheet");
    let page_title = props.header_text.text.to_owned();

    html! {
        <div class={stylesheet}>
            <div class={props.header_style.to_string()}>
                <header>{ format!("IdoKendo / {page_title}") }</header>
            </div>
        </div>
    }
}
