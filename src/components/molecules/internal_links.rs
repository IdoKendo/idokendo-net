use yew::prelude::*;

use crate::components::atoms::link_list::Link;
use crate::components::atoms::link_list::LinksList;

#[function_component(InternalLinks)]
pub fn internal_links() -> Html {
    let links = vec![
        Link {
            id: 0,
            title: "Home".to_string(),
            url: "/".to_string(),
        },
        Link {
            id: 1,
            title: "Blog".to_string(),
            url: "/blog".to_string(),
        },
    ];

    html! {
        <nav>
            <ul class="int-links">
                <LinksList links={links} />
            </ul>
        </nav>
    }
}
