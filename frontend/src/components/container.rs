use gloo_utils::document;
use reqwasm::http::Request;
use serde::Deserialize;
use web_sys::WheelEvent;
use yew::html::Scope;
use yew::{html, Component, Context, Html, Properties, Children};

use common::mongodb::structs::{Comment, ImageExpandState, ImageRequest, PostStats, Sort};

pub struct Container;

impl Component for Container {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={"container"}>
                <div class="navall">
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
                                            style="text-decoration: none;">{"LAYOUT2"}</a>
                                    </div>
                                </div>
                                <div class="indiv">
                                    <div>
                                        <a href="tags.html"
                                            class="link"
                                            style="text-decoration: none;">{"TAGS"}</a>
                                    </div>
                                </div>
                                <div class="indiv">
                                    <div>
                                        <a href="layout2.html"
                                            class="link"
                                            style="text-decoration: none;">{"ABOUT"}</a>
                                    </div>
                                </div>
                                <div class="indiv">
                                    <div>
                                        <a href="about.html"
                                            class="link"
                                            style="text-decoration: none;">{"SAMPLE"}</a>
                                    </div>
                                </div>
                            </div>
                        </center>
                    </div>
                </div>
            </div>
        }
    }
}
