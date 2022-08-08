use yew::{html, Component, Context, Html};

pub struct About;

impl Component for About {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2 style="position: relative; margin-top: 100px; margin-left: 200px;">{ "Welcome! to the Wholesome Yuri website" }</h2>
            </>
        }
    }
}
