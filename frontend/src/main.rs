mod components;

use components::posts::Posts;
use yew::{html, Component, Context, Html};

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
                <body style="background-color: black;">
                    <script type="module" src="https://unpkg.com/ionicons@5.5.2/dist/ionicons/ionicons.esm.js"></script>
                    <script nomodule=true src="https://unpkg.com/ionicons@5.5.2/dist/ionicons/ionicons.js"></script>
                    <div class={ "header-all" }>
                        <div class={ "header" }>
                            <h1>{ "Wholesome Yuri" }</h1>
                        </div>
                    </div>

                    <Posts/>
                </body>
            </>
        }
    }
}
fn main() {
    yew::start_app::<App>();
}
