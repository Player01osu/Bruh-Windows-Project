mod components;
mod pages;
mod routes;
mod session;

use crate::session::Session;
use components::{container::Container, template::Template};
use gloo_utils::window;
use routes::{switch, Route};
use yew::{html, Component, Context, Html};
use yew_router::{BrowserRouter, Switch};

pub struct App;

pub struct AppMsg(Session);

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Session::init(&ctx.link());
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let local_storage = window()
            .local_storage()
            .unwrap()
            .expect("Failed to get local storage");

        local_storage
            .set_item("public", &msg.0.user_pub)
            .expect("Failed to set private id");
        if !msg.0.user_priv.is_empty() {
            local_storage
                .set_item("private", &msg.0.user_priv.to_string())
                .expect("Failed to set public id");
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <BrowserRouter>
                    <Container/>
                    <Template>
                            <Switch<Route> render={Switch::render(switch)} />
                    </Template>
                </BrowserRouter>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
