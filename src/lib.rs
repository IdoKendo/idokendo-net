use gloo::console::log;
use gloo::events::EventListener;
use inflector::Inflector;
use std::ops::Deref;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use stylist::yew::styled_component;
use stylist::Style;
use web_sys::window;
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::*;

mod components;
mod router;

use components::atoms::header::{Header, HeaderStyle};
use components::molecules::external_links::ExternalLinks;
use components::molecules::internal_links::InternalLinks;
use router::{switch, Route};

const CSS: &str = include_str!("styles.css");

#[derive(Clone, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub email: String,
}

#[derive(Clone, PartialEq, Default)]
pub struct HeaderText {
    pub text: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let user_state = use_state(|| User::default());
    let header_state = use_state(|| HeaderText {
        text: "Home".to_owned(),
    });
    let stylesheet = Style::new(CSS).expect("Failed to create stylesheet");
    let first_load = use_state(|| true);

    let header_clone = header_state.clone();
    use_effect(move || {
        let window = window().expect("Failed to get window");
        if *first_load {
            let mut text = window
                .location()
                .pathname()
                .expect("Failed to get path name")
                .replace("/", "")
                .to_title_case();
            if text == "" {
                text = "Home".to_owned();
            }
            header_clone.set(HeaderText { text });
            first_load.set(false);
        }
        let document = window.document().expect("Failed to get document");
        let listener = EventListener::new(&document, "keydown", |event| {
            let event = event.dyn_ref::<KeyboardEvent>().unwrap_throw();
            match event.key().as_str() {
                ":" => log!("menu"),
                e => log!(e),
            };
        });
        || drop(listener)
    });

    let navigator_submit = {
        let header_state = header_state.clone();
        Callback::from(move |text| {
            header_state.set(HeaderText { text });
        })
    };

    let header_style = if user_state.username.clone() == "" {
        HeaderStyle::Normal
    } else if user_state.username.clone() == "ido" {
        HeaderStyle::Ok
    } else {
        HeaderStyle::Error
    };

    html! {
        <ContextProvider<User> context={user_state.deref().clone()}>
            <div class={stylesheet} id="idokendo">
                <Header header_style={header_style} header_text={header_state.deref().clone()} />
                <div class="content">
                    <BrowserRouter>
                        <InternalLinks onsubmit={navigator_submit} />
                        <ExternalLinks />
                        <Switch<Route> render={Switch::render(switch)} />
                    </BrowserRouter>
                </div>
            </div>
        </ContextProvider<User>>
    }
}
