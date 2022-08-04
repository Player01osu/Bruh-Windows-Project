use super::{fileupload::FileUpload, scrolltop::ScrollTop, sidebar::Sidebar};
use yew::{html, Component, Context, Html};

pub struct Container;

impl Component for Container {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={"container"}>
                <Sidebar/>
                <FileUpload/>
                <ScrollTop/>
            </div>
        }
    }
}
