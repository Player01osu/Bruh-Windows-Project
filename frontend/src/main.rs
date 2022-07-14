use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::html::Scope;
use yew::prelude::*;
use yew::{html, Component, Context, Html};

struct App {
    images: Vec<Image>,
}

enum Msg {
    ToggleExpando(usize),
    QueryImages(Vec<ImageRequest>),
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub enum ImageExpandState {
    Unfocus,
    Focus,
}

#[derive(Properties, Clone, PartialEq, Deserialize, Debug)]
pub struct Image {
    pub state: ImageExpandState,
    pub title: String,
    pub author: String,
    pub path: String,
    pub time: usize,
    pub tags: String,
    pub class: String,
    //pub width: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Id {
    #[serde(rename = "$oid")]
    oid: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ImageRequest {
    #[serde(rename = "_id")]
    _id: Id,
    title: String,
    author: String,
    op: String,
    time: usize,
    tags: Vec<String>,
    path: String,
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

impl App {
    pub fn view_images(&self, image_id: usize, image: &Image, link: &Scope<Self>) -> Html {
        html! {
            <div class="image-indiv">
                <img alt={format!("{} {}", image.author, image.title)}
                    src={format!("{}", image.path)}
                    class={format!("{}", image.class)}
                    //style={format!("max-width: {}px;", image.width)}
                    loading="lazy"
                    onclick={link.callback(move |_| Msg::ToggleExpando(image_id))}/>
                <div class="info">
                    <p>{format!("{}", image.tags)}</p>
                </div>
            </div>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let fetched_images: Vec<ImageRequest> = Request::get("/api/gallery_display")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            Msg::QueryImages(fetched_images)
        });
        let new_image_vec: Vec<Image> = Vec::new();

        return Self {
            images: new_image_vec,
        };
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleExpando(image_id) => {
                let image = self.images.get_mut(image_id).unwrap();

                image.toggle_expand();
                true
            }
            Msg::QueryImages(fetched_images) => {
                for image in fetched_images {
                    self.images.push(Image {
                        state: ImageExpandState::Unfocus,
                        title: image.title,
                        author: image.author,
                        path: image.path,
                        time: image.time,
                        tags: image.tags.concat(),
                        class: "yuri-img".to_string(),
                    })
                }
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
                <div class={ "header-all" }>
                    <div class={ "header" }>
                        <h1>{ "Wholesome Yuri" }</h1>
                    </div>
                </div>

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
            </>
        }
    }
}
fn main() {
    println!("hi");
    yew::start_app::<App>();
}
