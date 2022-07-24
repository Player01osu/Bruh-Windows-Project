use reqwasm::http::Request;
use serde::Deserialize;
use yew::html::Scope;
use yew::{html, Component, Context, Html, Properties};

use common::mongodb::structs::{Comment, ImageExpandState, ImageRequest, PostStats, Sort};

pub struct SortButtons;

impl Component for SortButtons {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="sort-buttons">

                </div>
            </>
        }
    }
}


pub enum ImageMessage {
    ToggleExpando(usize),
    QueryImages(Vec<ImageRequest>),
    ShowMore,
    Like(usize),
    No,
}

#[derive(PartialEq, Properties)]
pub struct PostProps {
    pub sort: String,
    pub document_height: f64,
    pub wheel_position: f64,
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
    pub time: usize,
    pub tags: Option<Vec<String>>,
    pub class: String,
    pub heart_state: ImageLiked,
    pub heart_class: String,
}

pub struct Posts {
    images: Vec<Image>,
    page: u16,
    sort: Sort,
    scroll_bottom_buffer: u16,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum ImageLiked {
    Liked,
    Unliked,
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
        html! {
            <div class="image-indiv">
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
                <img alt={format!("{} {}", image.author, image.title)}
                    src={format!(".{}", image.path)}
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
    type Properties = PostProps;

    fn create(ctx: &Context<Self>) -> Self {
        let sort = ctx.props().sort.clone();
        let new_image_vec: Vec<Image> = Vec::new();
        ctx.link().send_future(async move {
            // TODO: replace '1' w/ var that changes when scroll and 'new' w/ sort method
            let fetched_images: Vec<ImageRequest> = Request::get(format!{"/api/view-posts/1/{}", sort.clone()}.as_str())
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            ImageMessage::QueryImages(fetched_images)
        });

        let sort = match ctx.props().sort.as_str() {
            "new" => Sort::New,
            "top" => Sort::Top,
            "views" => Sort::Views,
            _ => Sort::New,
        };

        return Self {
            images: new_image_vec,
            page: 1,
            sort,
            scroll_bottom_buffer: 0,
        };
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ImageMessage::ToggleExpando(image_id) => {
                let image = self.images.get_mut(image_id).unwrap();

                image.toggle_expand();
                true
            },
            ImageMessage::QueryImages(fetched_images) => {
                for image in fetched_images {
                    self.images.push(Image {
                        oid: image._id.oid,
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
                        heart_state: ImageLiked::Unliked,
                        heart_class: "heart".to_string(),
                    })
                }
                true
            },
            ImageMessage::ShowMore => {
                match self.scroll_bottom_buffer {
                    0 => {
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
                        let mut new_image_vec: Vec<Image> = Vec::new();
                        self.images.append(&mut new_image_vec);
                        self.scroll_bottom_buffer = 20;

                        true
                    },
                    _ => {
                        self.scroll_bottom_buffer -= 1;
                        false
                    },
                }
            },
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
                                        "title": "{}",
                                        "path": "{}"
                                    }}"#,
                                    image.title,
                                    image.path
                                ))
                                .send()
                                .await
                                .expect("Failed to send put request (/api/like-post/)");
                            ImageMessage::No
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
                                        "title": "{}",
                                        "path": "{}"
                                    }}"#,
                                    image.title,
                                    image.path
                                ))
                                .send()
                                .await
                                .expect("Failed to send put request (/api/unlike-post/)");
                            ImageMessage::No
                        })
                    }
                };
                true
            },
            ImageMessage::No => false,
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
        //console_log!("{}, {}", ctx.props().wheel_position, ctx.props().document_height);
        match ctx.props().wheel_position / ctx.props().document_height > 0.8 {
            true => {
                ctx.link().send_message(ImageMessage::ShowMore);
                true
            },
            false => false,
        }
    }
}
