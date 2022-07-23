use web_sys::MouseEvent;
use yew::{html, Component, Context, Html};

pub struct ScrollTop;

pub enum ScrollTopMsg {
    Clicked,
}

impl Component for ScrollTop {
    type Message = ScrollTopMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ScrollTopMsg::Clicked => false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|event: MouseEvent| {
            event.view().unwrap().scroll_to_with_x_and_y(0.0, 0.0);
            ScrollTopMsg::Clicked
        });

        html! {
            <>
                <div>
                    <button style="position: absolute; margin-left: 1200px; margin-top: 400px;" {onclick}>
                        {"Click to scroll to top"}
                    </button>
                </div>
            </>
        }
    }
}
