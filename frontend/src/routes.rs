use crate::pages::{about::About, gallery::Gallery, home::Home, not_found::NotFound, tags::Tags, post::Post};
use yew::{html, Html};
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/gallery")]
    Gallery,
    #[at("/gallery/:post")]
    Post { post: String },
    #[at("/about")]
    About,
    #[at("/tags")]
    Tags,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home/> },
        Route::Gallery => html! { <Gallery/> },
        Route::Post { post } => html! { <Post post_id={post.clone()}/> },
        Route::About => html! { <About/> },
        Route::Tags => html! { <Tags/> },
        Route::NotFound => html! { <NotFound/> },
    }
}
