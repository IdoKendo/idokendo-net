use crate::components::atoms::custom_button::CustomButton;
use include_dir::{include_dir, Dir};
use yew::prelude::*;

const POSTS_DIR: Dir = include_dir!("posts");

#[derive(Debug)]
struct PostTitle {
    id: u32,
    title: String,
    date: String,
    html: String,
}

fn get_field(field: &str, contents: &str) -> String {
    if let Some(title_line) = contents.lines().find(|line| line.starts_with(field)) {
        if let Some(title) = title_line.split(':').nth(1) {
            return title.trim().trim_matches('"').to_owned();
        } else {
            return "".to_owned();
        }
    }
    "".to_owned()
}

#[derive(Clone, PartialEq, Default)]
struct DisplayedPost {
    id: i32,
}

#[function_component(Blog)]
pub fn blog() -> Html {
    let posts = use_memo((), |_| {
        let mut posts: Vec<PostTitle> = Vec::new();
        for (idx, file) in POSTS_DIR.files().enumerate() {
            let contents = file.contents_utf8().unwrap_or("");
            posts.push(PostTitle {
                id: idx as u32,
                title: get_field("title", contents),
                date: get_field("date", contents),
                html: markdown::to_html(contents),
            });
        }
        posts
    });

    let display_state = use_state(|| DisplayedPost { id: -1 });
    let post_select = {
        let display_state = display_state.clone();
        Callback::from(move |id: Option<String>| {
            if let Some(id) = id {
                display_state.set(DisplayedPost {
                    id: id.parse::<i32>().unwrap(),
                });
            } else {
                display_state.set(DisplayedPost { id: -1 });
            }
        })
    };

    let posts_list = posts
        .iter()
        .rev()
        .map(|post| {
            html! {
                <li>
                    <CustomButton label={post.title.clone()} message={post.id.to_string()} onclick={post_select.clone()}/> {" - "} {post.date.clone()}
                </li>
            }
        })
        .collect::<Html>();

    let post: Option<Html> = if display_state.id >= 0 {
        let html = posts[display_state.id as usize].html.clone();
        let html = html.split("</h2>").nth(1);
        match html {
            Some(article) => Some(yew::Html::from_html_unchecked(article.into())),
            None => None,
        }
    } else {
        None
    };

    html! {
        <>
            if display_state.id < 0 {
                <h1>{ "Blog" }</h1>
                {posts_list}
            } else {
                <CustomButton label="Blog" message="-1" onclick={post_select} />
                <h2>{posts[display_state.id as usize].title.clone()}</h2>
                <h3>{posts[display_state.id as usize].date.clone()}</h3>
                if let Some(post) = post {
                    {post}
                }
            }
        </>
    }
}
