use yew::{html, Component, Context, Html};

pub struct Header;

impl Component for Header {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <script type="module" src="https://unpkg.com/ionicons@5.5.2/dist/ionicons/ionicons.esm.js"></script>
                <script nomodule=true src="https://unpkg.com/ionicons@5.5.2/dist/ionicons/ionicons.js"></script>
                <div class={ "header-all" }>
                    <div class={ "header" }>
                        <h1>{ "Wholesome Yuri" }</h1>
                    </div>
                </div>
            </>
        }
    }
}
