mod about;
mod components;
mod gallery;
mod home;
mod not_found;
mod routes;
mod tags;

use routes::{switch, Route};
use yew::{html, Component, Context, Html};
use yew_router::{BrowserRouter, Switch};

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
