use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::Element;
use yew::html::Scope;
use yew::{html, Component, Context, Html, NodeRef, Properties};
use bson::oid::ObjectId;

use common::mongodb::structs::{
    ImageExpandState, ImageRequest, PostStats, Resolution, Source,
};
use yew_router::prelude::History;
use yew_router::scope_ext::RouterScopeExt;

use crate::routes::Route;

pub enum PostsMsg {
    ToggleExpando(usize),
    QueryImages(Vec<ImageRequest>),
    LoadPosts,
    Like(usize),
    ViewComments(usize),
    None,
}

#[derive(PartialEq, Properties)]
pub struct PostsProps {
    pub page_number: u16,
    pub query: PostQuery,
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
    pub comments: Option<ObjectId>,
    pub source: Source,
    pub resolution: Resolution,
    pub time: usize,
    pub tags: Option<Vec<String>>,
    pub style: String,
    pub class: String,
    pub heart_state: ImageLiked,
    pub heart_class: String,
}


pub struct Posts {
    images: Vec<Image>,
    page: u16,
    prev_succeed: bool,
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
                self.style = format!("");
                self.class = format!("yuri-img-clicked");
                self.state = ImageExpandState::Focus
            }
            ImageExpandState::Focus => {
                self.style = format!("");
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
    fn view_buttons(image: &Image, image_id: usize, link: &Scope<Self>) -> Html {
        html!{
            <div class="user-inter">
                    <button
                        type="button"
                        class={format!("{}", image.heart_class)}
                        onclick={link.callback(move |_| PostsMsg::Like(image_id))}
                    >
                        <ion-icon name="heart-outline"></ion-icon>
                    </button>
                    <button
                        type="button"
                        class="comments"
                        onclick={link.callback(move |_| PostsMsg::ViewComments(image_id))}
                    >
                        <ion-icon name="chatbubble-outline"></ion-icon>
                    </button>
            </div>
        }
    }

    fn view_tags(image: &Image) -> Html {
        html!{
            <div class="info">
                <p>
                {format!("{}", image
                    .tags
                    .as_ref()
                    .unwrap_or(&vec![String::new()])
                    .join(&", ")
                )}
                </p>
            </div>
        }
    }

    pub fn view_images(&self, image_id: usize, image: &Image, link: &Scope<Self>) -> Html {
        let buttons = Self::view_buttons(image, image_id, link);
        let tags = Self::view_tags(image);


        html! {
            <div class="image-indiv">
                { buttons }
                <img alt={format!("{} {}", image.author, image.title)}
                    src={format!(".{}", image.path)}
                    class={format!("{}", image.class)}
                    loading="lazy"
                    onclick={link.callback(move |_| PostsMsg::ToggleExpando(image_id))}
                    />
                { tags }
            </div>
        }
    }

    fn retrieve_posts(&self, link: &Scope<Self>, post_query: PostQuery) {
        // FIXME: Reference count?
        let request_builder = format!("{}?query={}", post_query.sort, post_query.query);
        let page = self.page;

        link.send_future(async move {
            let fetched_images: Vec<ImageRequest> =
                Request::get(format! {"/api/view-posts/{page}/{request_builder}"}.as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
            PostsMsg::QueryImages(fetched_images)
        });
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct PostQuery {
    #[serde(default = "default_sort")]
    pub sort: String,
    #[serde(rename = "q", default)]
    pub query: String,
}

fn default_sort() -> String {
    String::from("new")
}

impl Component for Posts {
    type Message = PostsMsg;
    type Properties = PostsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let posts = Self {
            images: Vec::default(),
            page: 1,
            prev_succeed: true,
        };

        let link = ctx.link();
        posts.retrieve_posts(link, ctx.props().query.clone());

        posts
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PostsMsg::ToggleExpando(image_id) => {
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

            PostsMsg::LoadPosts => {
                self.page = ctx.props().page_number;

                // Reset image vector on new page load/changed sort.
                if self.page == 1 {
                    self.images.clear();
                    self.prev_succeed = true;
                }

                // Retrieve posts only when the previous attempts succeed.
                if self.prev_succeed == true {
                    self.retrieve_posts(ctx.link(), ctx.props().query.clone());
                }
                true
            }

            PostsMsg::QueryImages(fetched_images) => {
                if !fetched_images.is_empty() {
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
                            style: "yuri-img".to_string(),
                            class: "yuri-img".to_string(),
                            heart_state: ImageLiked::Unliked,
                            heart_class: "heart".to_string(),
                        })
                    }
                    self.prev_succeed = true;
                } else {
                    self.prev_succeed = false;
                }
                true
            }

            PostsMsg::Like(image_id) => {
                let image = self.images.get_mut(image_id).unwrap();
                let image_oid = image.oid.clone();

                let request_uri = match image.toggle_like() {
                    true => String::from("/api/like-post"),
                    false => String::from("/api/unlike-post"),
                };

                ctx.link().send_future(async move {
                    Request::put(&request_uri)
                        .header("Content-Type", "application/json")
                        .body(&format!(
                            r#"
                            {{
                                "oid": "{}"
                            }}"#,
                            image_oid
                        ))
                        .send()
                        .await
                        .expect("Failed to send put request (/api/like-post/)");
                    PostsMsg::None
                });

                true
            }

            PostsMsg::ViewComments(image_id) => {
                let image = self.images.get_mut(image_id).unwrap();
                ctx.link().history().unwrap().push(Route::Post { post: image.comments.unwrap().to_string() });
                true
            }

            PostsMsg::None => false,
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
            <>
                <div class={ "images" }>
                    { posts }
                </div>
            </>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        ctx.link().send_message(PostsMsg::LoadPosts);
        true
    }
}
