mod components;

use components::posts::Posts;
use gloo_utils::document;
use web_sys::{console, Element, WheelEvent};
use yew::{html, use_state, Component, Context, Html, NodeRef, Children};

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class={ "header-all" }>
                    <div class={ "header" }>
                        <h1>{ "Wholesome Yuri" }</h1>
                    </div>
                </div>

                <div id="loadOnBottom" onwheel={|wheel_event: WheelEvent| {
                    let scroll_y = wheel_event
                        .view()
                        .unwrap()
                        .scroll_y()
                        .unwrap();
                    let page_height = document()
                        .get_element_by_id("loadOnBottom")
                        .expect("Element id not found")
                        .scroll_height();

                    if scroll_y / f64::from(page_height) > 0.9 {
                        console_log!("Bottom");
                    }
                }}>
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

                    <div>
                        <div class={ "images" }>
                            <Posts/>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
fn main() {
    console::log_1(&"hi".into());
    yew::start_app::<App>();
}
