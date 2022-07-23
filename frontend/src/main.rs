mod components;
mod gallery;
mod console_log;

use components::header::Header;
use components::container::Container;
use gallery::Gallery;
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
        Route::Home => html!{ <StaticPages/> },
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
        GalleryRoute::Top => html!{ <Gallery sort="top"/> },
        GalleryRoute::Views => html!{ <Gallery sort="views"/> },
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
                    <h2 style="position: relative; margin-top: 20px; margin-left: 80px;">{ "Welcome! to the Wholesome Yuri website" }</h2>
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
