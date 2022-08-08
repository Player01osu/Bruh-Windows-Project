use super::gallery::Gallery;
use super::{about::About, home::Home, not_found::NotFound, tags::Tags};
use yew::{html, Html};
use yew_router::prelude::*;
use yew_router::{Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/gallery")]
    Gallery,
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
        Route::About => html! { <About/> },
        Route::Tags => html! { <Tags/> },
        Route::NotFound => html! { <NotFound/> },
    }
}
