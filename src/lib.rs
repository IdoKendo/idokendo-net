use std::ops::Deref;

use gloo::console::log;
use stylist::yew::styled_component;
use stylist::Style;
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

#[styled_component(App)]
pub fn app() -> Html {
    let user_state = use_state(|| User::default());
    let title: Option<&str> = Some("Home");
    let stylesheet = Style::new(CSS).expect("Failed to create stylesheet");
    let first_load = use_state(|| true);

    use_effect(move || {
        if *first_load {
            log!("first load");
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
                <Header title={title} header_style={header_style} />
                <div class="content">
                    <BrowserRouter>
                        <InternalLinks />
                        <ExternalLinks />
                        <CustomForm onsubmit={custom_form_submit} />
                        <Switch<Route> render={Switch::render(switch)} />
                    </BrowserRouter>
                </div>
            </div>
        </ContextProvider<User>>
    }
}
