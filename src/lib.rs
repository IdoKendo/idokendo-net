use inflector::Inflector;
use std::ops::Deref;

use stylist::yew::styled_component;
use stylist::Style;
use web_sys::window;
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::*;

mod components;
mod router;

use components::atoms::header::{Header, HeaderStyle};
use components::molecules::custom_form::{CustomForm, Data};
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
        if *first_load {
            let mut text = window()
                .expect("Failed to get window")
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
        || {}
    });

    let custom_form_submit = {
        let user_state = user_state.clone();

        Callback::from(move |data: Data| {
            user_state.set(User {
                username: data.username,
                email: data.email,
            });
        })
    };

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
            <div class={stylesheet}>
                <Header header_style={header_style} header_text={header_state.deref().clone()} />
                <div class="content">
                    <BrowserRouter>
                        <InternalLinks onsubmit={navigator_submit} />
                        <ExternalLinks />
                        <CustomForm onsubmit={custom_form_submit} />
                        <Switch<Route> render={Switch::render(switch)} />
                    </BrowserRouter>
                </div>
            </div>
        </ContextProvider<User>>
    }
}
