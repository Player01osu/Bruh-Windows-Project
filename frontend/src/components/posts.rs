use super::sortbuttons::SortButtons;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::Element;
use yew::html::Scope;
use yew::{html, Component, Context, Html, NodeRef, Properties};
use yew_router::prelude::*;

use common::mongodb::structs::{
    Comment, ImageExpandState, ImageRequest, PostStats, Resolution, Source,
};
use yew_router::scope_ext::HistoryHandle;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub enum ImageMessage {
    ToggleExpando(usize),
    QueryImages(Vec<ImageRequest>),
    LoadPage,
    ShowMore,
    Like(usize),
    None,
}

#[derive(PartialEq, Properties)]
pub struct PostProps {
    pub document_height: f64,
    pub wheel_position: f64,
    pub gallery_node_ref: NodeRef,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Image {
    pub oid: String,
    pub state: ImageExpandState,
    pub title: String,
    pub author: String,
    pub op: String,
    pub path: String,
    pub stats: PostStats,
    pub comments: Option<Vec<Comment>>,
    pub source: Source,
    pub resolution: Resolution,
    pub time: usize,
    pub tags: Option<Vec<String>>,
    pub class: String,
    pub heart_state: ImageLiked,
    pub heart_class: String,
}

pub struct Posts {
    _history_handle: HistoryHandle,
    images: Vec<Image>,
    page: u16,
    scroll_bottom_buffer: u16,
    query: PostQuery,
    request_builder: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum ImageLiked {
    Liked,
    Unliked,
}

impl Image {
    pub fn toggle_expand(&mut self, _avail_width: i32) {
        match &self.state {
            ImageExpandState::Unfocus => {
                //let avail_width = avail_width as f32 * 0.71;

                //let margin_left = match self.resolution.width > 510 {
                //    true => -20,
                //    false => 0,
                //};
                self.class = format!("yuri-img-clicked");
                self.state = ImageExpandState::Focus
            }
            ImageExpandState::Focus => {
                self.class = format!("yuri-img");
                self.state = ImageExpandState::Unfocus
            }
        }
    }

    pub fn toggle_like(&mut self) -> bool {
        match &self.heart_state {
            ImageLiked::Liked => {
                self.heart_class = "heart".to_string();
                self.heart_state = ImageLiked::Unliked;
                false
            }
            ImageLiked::Unliked => {
                self.heart_class = "heart-liked".to_string();
                self.heart_state = ImageLiked::Liked;
                true
            }
        }
    }
}

impl Posts {
    pub fn view_images(&self, image_id: usize, image: &Image, link: &Scope<Self>) -> Html {
        let buttons = html! {
            <div class="user-inter">
                    <button type="button"
                        class={format!("{}", image.heart_class)}
                        onclick={link.callback(move |_| ImageMessage::Like(image_id))}>
                        <ion-icon name="heart-outline"></ion-icon>
                    </button>
                    <button type="button" class="comments">
                        <ion-icon name="chatbubble-outline"></ion-icon>
                    </button>
            </div>
        };

        let tags = html! {
                <div class="info">
                    <p>
                    {format!("{}", image.tags
                        .as_ref()
                        .unwrap_or(&vec![String::new()])
                        .join(&", ")
                    )}
                    </p>
                </div>
        };

        html! {
            <div class="image-indiv">
                { buttons }
                <img alt={format!("{} {}", image.author, image.title)}
                    src={format!(".{}", image.path)}
                    class={format!("{}", image.class)}
                    loading="lazy"
                    onclick={link.callback(move |_| ImageMessage::ToggleExpando(image_id))}
                    />
            { tags }
            </div>
        }
    }

    fn retrieve_posts(&mut self, link: &Scope<Self>) {
        let mut sort = link
            .location()
            .unwrap()
            .pathname()
            .split_once("gallery/")
            .unwrap_or(("", "new"))
            .1
            .to_string();
        // FIXME: this kinda hacky
        if sort.is_empty() {
            sort = "new".to_string();
        }

        self.query = link.location().unwrap().query::<PostQuery>().unwrap();
        // FIXME: Reference count?
        self.request_builder = format!("{sort}?query={}", self.query.query);
        let request_builder = self.request_builder.clone();

        link.send_future(async move {
            let fetched_images: Vec<ImageRequest> =
                Request::get(format! {"/api/view-posts/1/{request_builder}"}.as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
            ImageMessage::QueryImages(fetched_images)
        });
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PostQuery {
    #[serde(rename = "q", default)]
    pub query: String,
}

impl Component for Posts {
    type Message = ImageMessage;
    type Properties = PostProps;

    fn create(ctx: &Context<Self>) -> Self {
        let history_listener = ctx
            .link()
            .add_history_listener(ctx.link().callback(|_| ImageMessage::LoadPage))
            .unwrap();

        let posts = Self {
            _history_handle: history_listener,
            images: Vec::default(),
            page: 1,
            scroll_bottom_buffer: 0,
            query: PostQuery::default(),
            request_builder: String::from("new"),
        };
        // FIXME FIXME bruv why
        ctx.link().send_message(ImageMessage::LoadPage);

        posts
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ImageMessage::ToggleExpando(image_id) => {
                let image = self.images.get_mut(image_id).unwrap();
                let avail_width = ctx
                    .props()
                    .gallery_node_ref
                    .cast::<Element>()
                    .unwrap()
                    .client_width();

                image.toggle_expand(avail_width);
                true
            }

            ImageMessage::LoadPage => {
                self.images.clear();
                self.retrieve_posts(ctx.link());
                true
            }

            ImageMessage::QueryImages(fetched_images) => {
                for image in fetched_images {
                    self.images.push(Image {
                        oid: image._id.oid,
                        state: ImageExpandState::Unfocus,
                        title: image.title,
                        author: image.author,
                        op: image.op,
                        source: image.source,
                        resolution: image.resolution,
                        path: image.path,
                        stats: image.stats,
                        time: image.time,
                        tags: image.tags,
                        comments: image.comments,
                        class: "yuri-img".to_string(),
                        heart_state: ImageLiked::Unliked,
                        heart_class: "heart".to_string(),
                    })
                }
                true
            }

            ImageMessage::ShowMore => {
                match self.scroll_bottom_buffer {
                    0 => {
                        self.page += 1;
                        let api_request =
                            format!("/api/view-posts/{}/{}", self.page, self.request_builder);
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
                        let mut new_image_vec: Vec<Image> = Vec::new();
                        self.images.append(&mut new_image_vec);
                        self.scroll_bottom_buffer = 20;

                        true
                    }
                    _ => {
                        self.scroll_bottom_buffer -= 1;
                        false
                    }
                }
            }

            ImageMessage::Like(image_id) => {
                let image = self.images.get_mut(image_id).unwrap();

                match image.toggle_like() {
                    true => {
                        let image = image.clone();
                        ctx.link().send_future(async move {
                            Request::put(&format!("/api/like-post"))
                                .header("Content-Type", "application/json")
                                .body(&format!(
                                    r#"
                                    {{
                                        "oid": "{}"
                                    }}"#,
                                    image.oid
                                ))
                                .send()
                                .await
                                .expect("Failed to send put request (/api/like-post/)");
                            ImageMessage::None
                        })
                    }
                    false => {
                        let image = image.clone();
                        ctx.link().send_future(async move {
                            Request::put(&format!("/api/unlike-post"))
                                .header("Content-Type", "application/json")
                                .body(&format!(
                                    r#"
                                    {{
                                        "oid": "{}"
                                    }}"#,
                                    image.oid
                                ))
                                .send()
                                .await
                                .expect("Failed to send put request (/api/unlike-post/)");
                            ImageMessage::None
                        })
                    }
                };
                true
            }

            ImageMessage::None => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let posts = self
            .images
            .iter()
            .enumerate()
            .map(|(id, image)| {
                html! {
                    { self.view_images(id, image, ctx.link()) }
                }
            })
            .collect::<Html>();

        html! {
            <>  <center>
                    <SortButtons/>
                    <div class={ "images" }>
                        { posts }
                    </div>
                </center>
            </>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        match ctx.props().wheel_position / ctx.props().document_height > 0.8 {
            true => {
                ctx.link().send_message(ImageMessage::ShowMore);
                true
            }
            false => true,
        }
    }
}
