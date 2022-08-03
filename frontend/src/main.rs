mod components;
mod gallery;
mod about;
mod tags;
mod not_found;

use components::header::Header;
use components::container::Container;
use about::About;
use gallery::{Gallery, Sort};
use not_found::NotFound;
use tags::Tags;
use yew::{html, Component, Context, Html};
use yew_router::{BrowserRouter, Switch, Routable};
use yew_router::prelude::*;

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
        Route::Home => html!{ <StaticPages/> },
        Route::GalleryRouter => html!{
            <Switch<GalleryRoute> render={Switch::render(gallery_switch)} />
        },
        Route::Gallery => html!{ <Gallery sort={Sort::New}/> },
        Route::About => html! { <About/> },
        Route::Tags => html! { <Tags/> },
        Route::NotFound => html!{ <NotFound/> },
    }
}

pub fn gallery_switch(route: &GalleryRoute) -> Html {
    match route {
        GalleryRoute::New => html!{ <Gallery sort={Sort::New}/> },
        GalleryRoute::Top => html!{ <Gallery sort={Sort::Top}/> },
        GalleryRoute::Views => html!{ <Gallery sort={Sort::Views}/> },
        GalleryRoute::NotFound => html!{
            <Redirect<Route> to={Route::NotFound}/>
        }
    }
}

pub enum StaticMsg {
    LoadPage(Html),
}

struct StaticPages {
    page: Html,
}

impl Component for StaticPages {
    type Message = StaticMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            page: html!{},
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            StaticMsg::LoadPage(page_html) => {
                self.page = page_html;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <>
                <body style="background-color: black;">
                    <Header/>
                    <Container/>
                    <h2 style="position: relative; margin-top: 100px; margin-left: 200px;">{ "Welcome! to the Wholesome Yuri website" }</h2>
                </body>
            </>
        }
    }
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
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
