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
            ScrollTopMsg::Clicked => false,
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
                    <button style="position: fixed; background-color: #c054c2;
                    opacity: 0.5;
                    color: 9a9996;
                    width: 120px;
                    font-size: 15px;
                    border: none; top: 85px; right: 140px; z-index: 69420;" {onclick}>
                        {"Up"}
                    </button>
                </div>
            </>
        }
    }
}
