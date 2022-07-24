use yew::{html, Component, Context, Html, Properties};

pub struct Sidebar {
    visibility: SidebarVisibility,
    style: String,
}

pub enum SidebarMsg {
    Toggle,
}

pub enum SidebarVisibility {
    Show,
    Hidden,
}

#[derive(PartialEq, Properties)]
pub struct LinkProps {
    link: String,
    text: String,
}

pub struct Links;

impl Component for Links {
    type Properties = LinkProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.props().link.clone();
        let text = ctx.props().text.clone();
        html!{
            <div class="indiv">
                <div>
                    <a href={format!("{}", link)}
                        class="link"
                        style="text-decoration: none;">{text}
                    </a>
                </div>
            </div>
        }
    }
}

impl Component for Sidebar {
    type Properties = ();
    type Message = SidebarMsg;
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            visibility: SidebarVisibility::Show,
            style: String::new(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SidebarMsg::Toggle => {
                match self.visibility {
                    SidebarVisibility::Show => {
                        self.style = "display: none;".to_string();
                        self.visibility = SidebarVisibility::Hidden;
                    }
                    SidebarVisibility::Hidden => {
                        self.style = String::new();
                        self.visibility = SidebarVisibility::Show;
                    }
                }
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| SidebarMsg::Toggle);
        html! {
            <>
                <button style="margin-left: 200px; margin-top: 200px;" {onclick}> {"Click to hide"} </button>

                <div class="navall" style={format!("{}", &self.style)}>
                    <div class="nav">
                            <form action="" class="search-bar">
                                <input type="text" class="search" placeholder="search tag or somth" name="q"/>
                            </form>
                            <div class="nav-img">
                                <div>
                                    <img class="imge" src="../assets/img/blah.jpg" alt="nav-img"/>
                                </div>
                            </div>
                        <center>
                            <div class="links">
                                <Links link="/" text="HOME"/>
                                <Links link="gallery" text="GALLERY"/>
                                <Links link="tags" text="TAGS"/>
                                <Links link="about" text="ABOUT"/>
                                <Links link="github" text="GITHUB"/>
                            </div>
                        </center>
                    </div>
                </div>
            </>
        }
    }
}
