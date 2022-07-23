use web_sys::MouseEvent;
use yew::{html, Component, Context, Html};

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
                                    <img class="imge" src="assets/img/blah.jpg" alt="nav-img"/>
                                </div>
                            </div>
                        <center>
                            <div class="links">
                                <div class="indiv">
                                    <div>
                                        <a href="layout2.html"
                                            class="link"
                                            style="text-decoration: none;">{"LAYOUT2"}
                                        </a>
                                    </div>
                                </div>
                                <div class="indiv">
                                    <div>
                                        <a href="tags.html"
                                            class="link"
                                            style="text-decoration: none;">{"TAGS"}
                                        </a>
                                    </div>
                                </div>
                                <div class="indiv">
                                    <div>
                                        <a href="layout2.html"
                                            class="link"
                                            style="text-decoration: none;">{"ABOUT"}
                                        </a>
                                    </div>
                                </div>
                                <div class="indiv">
                                    <div>
                                        <a href="about.html"
                                            class="link"
                                            style="text-decoration: none;">{"SAMPLE"}
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </center>
                    </div>
                </div>
            </>
        }
    }
}
