use gloo::console::log;
use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

mod components;

use components::atoms::header::{Header, HeaderStyle};
use components::atoms::intro::Intro;
use components::molecules::external_links::ExternalLinks;
use components::molecules::internal_links::InternalLinks;

const CSS: &str = include_str!("styles.css");

#[styled_component(App)]
pub fn app() -> Html {
    let title: Option<&str> = Some("Home");
    let stylesheet = Style::new(CSS).expect("Failed to create stylesheet");
    let main_title_load = Callback::from(|message: String| {
        log!(message);
    });

    html! {
        <div class={stylesheet}>
            <Header title={title} header_style={HeaderStyle::Normal} on_load={main_title_load} />
            <div class="content">
                <InternalLinks />
                <ExternalLinks />
                <main><Intro /></main>
            </div>
        </div>
    }
}
