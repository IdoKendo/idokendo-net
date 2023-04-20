use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

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
    pub title: Option<String>,
    pub header_style: HeaderStyle,
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

    html! {
        <div class={stylesheet}>
            <div class={props.header_style.to_string()}>
                if let Some(page_title) = &props.title {
                    <header>{ format!("IdoKendo / {page_title}") }</header>
                } else {
                    <header> { "IdoKendo / Where are we?" }</header>
                }
            </div>
        </div>
    }
}
