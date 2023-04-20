use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MyLink {
    pub id: usize,
    pub title: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub links: Vec<MyLink>,
}

#[function_component(LinksList)]
pub fn links(Props { links }: &Props) -> Html {
    links
        .iter()
        .map(|link| {
            html! {
                <li key={link.id}>
                    <a href={link.url.clone()}>{link.title.clone()}</a>
                </li>
            }
        })
        .collect::<Html>()
}
