use yew::prelude::*;

#[function_component(Intro)]
pub fn intro() -> Html {
    html! {
        <>
        <h1>{ "About Me" }</h1>
        <p>{ "Hi! I am Ido. I am a software developer with a lot of passion to processes, code quality, immaculate pipelines and infrastructures." }</p>
        <p>{ "I love trying out new technologies and frameworks, while both improving my skills and gaining new ones." }</p>
        <p>{ "My main stack is Python, which I have been involved with, in one way or another, since 2010. (Oh my god, now I feel old)" }</p>
        <p>{ "Recently, as many others, I've gained some interest in Rust, and am exploring it on my free time. This site itself is written in Rust using the Yew framework!" }</p>
        <p>{ "I'm also an nvim fanatic, constantly tinkering and messing with my config files, if you know of any cool plugin, hit me up!" }</p>
        <p>{ "If you want to reach me, either on professional topics, or just to chat about Devil May Cry, you can reach me on my email or via LinkedIn." }</p>
        </>
    }
}
