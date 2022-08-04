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
                <div style="margin-top: 4px; position: sticky; float: right; top: 0; margin-right: 250px; width: 120px;">
                    <button style="
                    position: sticky;
                    top: 0;
                    background-color: #c054c2;
                    opacity: 0.5;
                    color: 9a9996;
                    width: 120px;
                    font-size: 15px;
                    border: none; "
                    {onclick}>
                        {"Up"}
                    </button>
                </div>
            </>
        }
    }
}
