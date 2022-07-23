use super::components::posts::Posts;
use super::components::header::Header;
use yew::{html, Component, Context, Html, Properties};
use yew_router::{BrowserRouter, Switch, Routable};

pub struct Gallery;
pub struct GalleryTop;
pub struct GalleryViews;

#[derive(Properties, PartialEq)]
pub struct GalleryProps {
    pub sort: String,
}

impl Component for Gallery {
    type Properties = GalleryProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <body style="background-color: black;">
                    <Header/>
                    <Posts sort={ctx.props().sort.clone()}/>
                </body>
            </>
        }
    }
}

impl Component for GalleryTop {
    type Properties = GalleryProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! { <Gallery sort={ctx.props().sort.clone()}/> }
    }
}

impl Component for GalleryViews {
    type Properties = GalleryProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! { <Gallery sort={ctx.props().sort.clone()}/> }
    }
}
