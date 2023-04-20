use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(InternalLinks)]
pub fn internal_links() -> Html {
    html! {
        <nav>
            <ul class="int-links">
                <li>
                    <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Blog}>{"Blog"}</Link<Route>>
                </li>
            </ul>
        </nav>
    }
}
