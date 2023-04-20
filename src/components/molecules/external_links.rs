use yew::prelude::*;

use crate::components::atoms::link_list::Link;
use crate::components::atoms::link_list::LinksList;

#[function_component(ExternalLinks)]
pub fn external_links() -> Html {
    let links = vec![
        Link {
            id: 0,
            title: "LinkedIn".to_string(),
            url: "https://www.linkedin.com/in/ido-slonimsky-64a15755/".to_string(),
        },
        Link {
            id: 1,
            title: "Email".to_string(),
            url: "mailto:ido.slonimsky@gmail.com".to_string(),
        },
        Link {
            id: 2,
            title: "GitHub".to_string(),
            url: "https://github.com/IdoKendo".to_string(),
        },
        Link {
            id: 3,
            title: "Mastodon".to_string(),
            url: "https://fosstodon.org/@IdoKendo".to_string(),
        },
    ];
    html! {
        <nav>
            <ul>
                <LinksList links={links} />
            </ul>
        </nav>
    }
}
