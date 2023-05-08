use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pages::blog::Blog;
use crate::components::pages::home::Home;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/idokendo-net")]
    Home,
    #[at("idokendo-net/blog")]
    Blog,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Blog => html! { <Blog /> },
    }
}
