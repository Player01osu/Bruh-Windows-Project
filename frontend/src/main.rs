mod components;
mod gallery;

use std::fmt::Display;

use components::posts::Posts;
use gallery::{Gallery, GalleryViews, GalleryTop};
use yew::{html, Component, Context, Html};
use yew_router::{BrowserRouter, Switch, Routable};

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
}

#[derive(Clone, Routable, PartialEq)]
pub enum GalleryRoute {
    #[at("/gallery/new")]
    New,
    #[at("/gallery/top")]
    Top,
    #[at("/gallery/views")]
    Views,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => todo!(),
        Route::GalleryRouter => html!{
            <Switch<GalleryRoute> render={Switch::render(gallery_switch)} />
        },
        Route::Gallery => html!{ <Gallery sort="new"/> },
        Route::About => todo!(),
        Route::Tags => todo!(),
    }
}

pub fn gallery_switch(route: &GalleryRoute) -> Html {
    match route {
        GalleryRoute::New => html!{ <Gallery sort="new"/> },
        GalleryRoute::Top => html!{ <GalleryTop sort="top"/> },
        GalleryRoute::Views => html!{ <GalleryViews sort="views"/> },
    }
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}
fn main() {
    yew::start_app::<App>();
}
