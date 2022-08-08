use yew::{html, Component, Context, Html};

pub enum HomeMsg {
    LoadPage(Html),
}

pub struct Home {
    page: Html,
}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { page: html! {} }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::LoadPage(page_html) => {
                self.page = page_html;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2 style="position: relative; margin-top: 100px; margin-left: 200px;">
                    { "Welcome! to the Wholesome Yuri website" }
                </h2>
            </>
        }
    }
}
