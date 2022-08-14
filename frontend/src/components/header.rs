use yew::{html, Component, Context, Html, Properties};

pub struct Header;

#[derive(PartialEq, Eq, Properties)]
pub struct HeaderProps {
    pub is_collapsed: bool,
}

impl Component for Header {
    type Properties = HeaderProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let header_all_class = match ctx.props().is_collapsed {
            false => String::from("header-all-collapsed"),
            true => String::from("header-all"),
        };

        let nav_header_class = match ctx.props().is_collapsed {
            false => String::from("nav-header-collapsed"),
            true => String::from("nav-header"),
        };

        html! {
            <>
                <div class={ &header_all_class }>
                    <div class={ "header" }>
                        <h1>{ "Wholesome Yuri" }</h1>
                    </div>
                </div>

                <div class={ &nav_header_class }>

                </div>
            </>
        }
    }
}
