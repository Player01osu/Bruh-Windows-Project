use reqwasm::http::Request;
use serde::Deserialize;
use yew::html::Scope;
use yew::{html, Component, Context, Html};

use common::mongodb::structs::{Comment, ImageExpandState, ImageMessage, ImageRequest, PostStats, Sort};

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
                <div class={ "images" }>
                    { posts }
                </div>
            </>
        }
    }
}
