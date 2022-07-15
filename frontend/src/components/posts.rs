use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::html::Scope;
use yew::prelude::*;
use yew::{html, Component, Context, Html};

pub struct Posts {
    images: Vec<Image>,
}

pub enum ImageMessage {
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
}

#[derive(Deserialize, Serialize, Debug)]
struct Id {
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

impl Posts {
    pub fn view_images(&self, image_id: usize, image: &Image, link: &Scope<Self>) -> Html {
        html! {
            <div class="image-indiv">
                <img alt={format!("{} {}", image.author, image.title)}
                    src={format!("{}", image.path)}
                    class={format!("{}", image.class)}
                    //style={format!("max-width: {}px;", image.width)}
                    loading="lazy"
                    onclick={link.callback(move |_| ImageMessage::ToggleExpando(image_id))}/>
                <div class="info">
                    <p>{format!("{}", image.tags)}</p>
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
            let fetched_images: Vec<ImageRequest> = Request::get("/api/gallery_display")
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
        };
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                <div class={ "images" }>
                    { posts }
                </div>
            </>
        }
    }
}