use super::{header::Header, sidebar::Sidebar};
use yew::{html, Callback, Children, Component, Context, ContextProvider, Html, Properties};

pub struct Template {
    class: String,
    sidebar_toggle: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    pub callback: Callback<TemplateMsg>,
}

pub enum TemplateMsg {
    ToggleSidebar,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TemplateProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Template {
    type Properties = TemplateProps;
    type Message = TemplateMsg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            class: String::default(),
            sidebar_toggle: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TemplateMsg::ToggleSidebar => {
                match self.sidebar_toggle {
                    true => {
                        self.class = "sidebar-collapsed".to_string();
                        self.sidebar_toggle = false;
                    }
                    false => {
                        self.class = String::default();
                        self.sidebar_toggle = true;
                    }
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let body = Body {
            callback: ctx.link().callback(|_| TemplateMsg::ToggleSidebar),
        };

        let body_style = &self.class;

        html! {
            <>
                <div class={body_style}>
                    <Header is_collapsed={self.class.is_empty()}/>
                    <ContextProvider<Body> context={body}>
                        <div class="below">
                            <Sidebar/>
                            { for ctx.props().children.iter() }
                        </div>
                    </ContextProvider<Body>>
                </div>
            </>
        }
    }
}
