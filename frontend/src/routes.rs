use super::gallery::{Gallery, Sort};
use super::{about::About, home::Home, not_found::NotFound, tags::Tags};
use yew::{html, Html};
use yew_router::prelude::*;
use yew_router::{Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/gallery/:sort")]
    GalleryRouter,
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

#[derive(Clone, Routable, PartialEq)]
pub enum GalleryRoute {
    #[at("/gallery/new")]
    New,
    #[at("/gallery/top")]
    Top,
    #[at("/gallery/views")]
    Views,
    #[not_found]
    #[at("/gallery/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home/> },
        Route::GalleryRouter => html! {
            <Switch<GalleryRoute> render={Switch::render(gallery_switch)} />
        },
        Route::Gallery => html! { <Gallery/> },
        Route::About => html! { <About/> },
        Route::Tags => html! { <Tags/> },
        Route::NotFound => html! { <NotFound/> },
    }
}

pub fn gallery_switch(route: &GalleryRoute) -> Html {
    match route {
        GalleryRoute::New => html! { <Gallery/> },
        GalleryRoute::Top => html! { <Gallery/> },
        GalleryRoute::Views => html! { <Gallery/> },
        GalleryRoute::NotFound => html! {
            <Redirect<Route> to={Route::NotFound}/>
        },
    }
}
