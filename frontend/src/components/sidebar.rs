use crate::{routes::GalleryRoute, Route};
use serde::Serialize;
use web_sys::{FormData, HtmlFormElement};
use yew::{html, Callback, Component, Context, Html, Properties, TargetCast};
use yew_router::prelude::*;

use super::template::{Body, TemplateMsg};

pub struct Sidebar {
    visibility: SidebarVisibility,
    style: String,
}

pub enum SidebarMsg {
    Toggle,
    None,
}

pub enum SidebarVisibility {
    Show,
    Hidden,
}

#[derive(PartialEq, Properties)]
pub struct LinkProps {
    link: String,
    text: String,
    route: Route,
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
        let text = ctx.props().text.clone();
        let route = ctx.props().route.clone();
        html! {
            <div class="indiv">
                <div>
                    <Link<Route> to={route} classes="link">{ format!("{text}") }</Link<Route>>
                </div>
            </div>
        }
    }
}

#[derive(Serialize)]
pub struct QueryStruct {
    query: String,
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (body, _) = ctx.link().context::<Body>(Callback::noop()).unwrap();
        match msg {
            SidebarMsg::Toggle => {
                match self.visibility {
                    SidebarVisibility::Show => {
                        self.style = "display: none;".to_string();
                        body.callback.emit(TemplateMsg::ToggleSidebar);
                        self.visibility = SidebarVisibility::Hidden;
                    }
                    SidebarVisibility::Hidden => {
                        self.style = String::default();
                        body.callback.emit(TemplateMsg::ToggleSidebar);
                        self.visibility = SidebarVisibility::Show;
                    }
                }
                true
            }
            SidebarMsg::None => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().clone().history().unwrap();

        let onclick = ctx.link().callback(|_| SidebarMsg::Toggle);
        let onsubmit = {
            ctx.link().callback(move |event: web_sys::FocusEvent| {
                event.prevent_default();
                let form = event.target_unchecked_into::<HtmlFormElement>();
                let data = FormData::new_with_form(&form).unwrap();

                let query = data.get("q").as_string().expect("Fucking parse this mf");
                let query = QueryStruct { query };

                history.push_with_query(GalleryRoute::New, query).unwrap();
                SidebarMsg::None
            })
        };

        let links = html! {
            <>
                <div class="links">
                    <Links link="/" text="HOME" route={Route::Home}/>
                    <Links link="/gallery" text="GALLERY" route={Route::Gallery}/>
                    <Links link="/tags" text="TAGS" route={Route::Tags}/>
                    <Links link="/about" text="ABOUT" route={Route::About}/>
                    <div class="indiv">
                        <div>
                            <a href="https://github.com/player01osu/yuri-web"
                                class="link"
                                style="text-decoration: none;">
                                    {"GITHUB"}
                            </a>
                        </div>
                    </div>
                </div>
            </>
        };

        html! {
            <>
                <button style="margin-left: 200px; margin-top: 200px;" {onclick}> {"Click to hide"} </button>

                <div class="navall" style={format!("{}", &self.style)}>
                    <div class="nav">
                            <form
                                action=""
                                class="search-bar"
                                {onsubmit}
                            >
                                <input
                                    type="text"
                                    class="search"
                                    placeholder="search tag or somth"
                                    name="q"/>
                            </form>
                            <div class="nav-img">
                                <div>
                                    <img class="imge" src="../assets/img/blah.jpg" alt="nav-img"/>
                                </div>
                            </div>
                        <center>
                            { links }
                        </center>
                    </div>
                </div>
            </>
        }
    }
}
