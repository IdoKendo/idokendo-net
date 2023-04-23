use include_dir::{include_dir, Dir};
use log::info;
use yew::prelude::*;

const POSTS_DIR: Dir = include_dir!("posts");

#[derive(Debug)]
struct PostTitle {
    title: String,
    date: String,
    html: String,
}

fn get_field(field: &str, contents: &str) -> String {
    if let Some(title_line) = contents.lines().find(|line| line.starts_with(field)) {
        if let Some(title) = title_line.split(":").nth(1) {
            return title.trim().trim_matches('"').to_owned();
        } else {
            return "".to_owned();
        }
    }
    "".to_owned()
}

#[function_component(Blog)]
pub fn blog() -> Html {
    let mut posts: Vec<PostTitle> = Vec::new();

    for file in POSTS_DIR.files() {
        let contents = file.contents_utf8().unwrap_or("");
        posts.push(PostTitle {
            title: get_field("title", contents),
            date: get_field("date", contents),
            html: markdown::to_html(contents),
        });
    }
    info!("{:?}", posts);
    html! {
        <h1>{ "Blog" }</h1>
    }
}
