mod pages;
mod components;
mod routes;

use components::{template::Template, container::Container};
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
                <Template>
                    <Container/>
                        <Switch<Route> render={Switch::render(switch)} />
                </Template>
            </BrowserRouter>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
