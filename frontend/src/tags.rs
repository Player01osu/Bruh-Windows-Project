use super::components::posts::Posts;
use super::components::container::Container;
use super::components::header::Header;
use gloo_utils::document;
use web_sys::WheelEvent;
use yew::{html, Component, Context, Html, Properties};

pub struct Tags;

impl Component for Tags {
    type Properties = ();
    type Message = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
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
