use gloo_utils::document;
use reqwasm::http::Request;
use serde::Deserialize;
use web_sys::WheelEvent;
use yew::html::Scope;
use yew::{html, Component, Context, Html, Properties, Children};

use common::mongodb::structs::{Comment, ImageExpandState, ImageRequest, PostStats, Sort};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Clone)]
pub enum ImageMessage {
    ToggleExpando(usize),
    QueryImages(Vec<ImageRequest>),
    ShowMore,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Image {
    pub state: ImageExpandState,
    pub title: String,
    pub author: String,
    pub op: String,
    pub path: String,
    pub stats: PostStats,
    pub comments: Option<Vec<Comment>>,
    pub time: usize,
    pub tags: Option<Vec<String>>,
    pub class: String,
}

pub struct Posts {
    images: Vec<Image>,
    page: u16,
    sort: Sort,
}

impl Image {
    pub fn toggle_expand(&mut self) {
        match &self.state {
            ImageExpandState::Unfocus => {
                self.class = "yuri-img-clicked".to_string();
                self.state = ImageExpandState::Focus
            }
            ImageExpandState::Focus => {
                self.class = "yuri-img".to_string();
                self.state = ImageExpandState::Unfocus
            }
        }
    }
}

impl Posts {
    pub fn view_images(&self, image_id: usize, image: &Image, link: &Scope<Self>) -> Html {
        html! {
            <div class="image-indiv">
                <img alt={format!("{} {}", image.author, image.title)}
                    src={format!("{}", image.path)}
                    class={format!("{}", image.class)}
                    //style={format!("max-width: {}px;", image.width)}
                    loading="lazy"
                    onclick={link.callback(move |_| ImageMessage::ToggleExpando(image_id))}
                    />
                <div class="info">
                    <p>
                    {format!("{}", image.tags
                        .as_ref()
                        .unwrap_or(&vec![String::new()])
                        .join(&", ")
                    )}
                    </p>
                </div>
            </div>
        }
    }
}

impl Component for Posts {
    type Message = ImageMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            // TODO: replace '1' w/ var that changes when scroll and 'new' w/ sort method
            let fetched_images: Vec<ImageRequest> = Request::get("/api/view-posts/1/new")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            ImageMessage::QueryImages(fetched_images)
        });
        let new_image_vec: Vec<Image> = Vec::new();

        return Self {
            images: new_image_vec,
            page: 1,
            sort: Sort::New,
        };
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ImageMessage::ToggleExpando(image_id) => {
                let image = self.images.get_mut(image_id).unwrap();

                image.toggle_expand();
                true
            }

            ImageMessage::QueryImages(fetched_images) => {
                for image in fetched_images {
                    self.images.push(Image {
                        state: ImageExpandState::Unfocus,
                        title: image.title,
                        author: image.author,
                        op: image.op,
                        path: image.path,
                        stats: image.stats,
                        time: image.time,
                        tags: image.tags,
                        comments: image.comments,
                        class: "yuri-img".to_string(),
                    })
                }
                true
            }
            ImageMessage::ShowMore => {
                self.page += 1;
                let api_request = match self.sort {
                    Sort::New => format!("/api/view-posts/{}/new", self.page),
                    Sort::Top => format!("/api/view-posts/{}/top", self.page),
                    Sort::Views => format!("/api/view-posts/{}/views", self.page),
                };
                ctx.link().send_future(async move {
                    // TODO: replace '1' w/ var that changes when scroll and 'new' w/ sort method
                    let fetched_images: Vec<ImageRequest> = Request::get(&api_request)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    ImageMessage::QueryImages(fetched_images)
                });
                let new_image_vec: Vec<Image> = Vec::new();

                self.images = new_image_vec;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let posts: Html = self
            .images
            .iter()
            .enumerate()
            .map(|(id, image)| {
                let image_list = self.view_images(id, image, ctx.link());
                html! {
                    <div>
                        {image_list}
                    </div>
                }
            })
            .collect();
        html! {
            <>
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
                    <div class={ "images" }>
                        { posts }
                    </div>
                </div>
            </>
        }
    }
}
