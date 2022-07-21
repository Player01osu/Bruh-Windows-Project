mod components;

use components::posts::Posts;
use yew::{html, Component, Context, Html};

use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
            <>
                <div class={ "header-all" }>
                    <div class={ "header" }>
                        <h1>{ "Wholesome Yuri" }</h1>
                    </div>
                </div>

                <Posts/>

            </>
        }
    }
}
fn main() {
    yew::start_app::<App>();
}
