use std::rc::Rc;

use reqwasm::http::Request;
use serde::Deserialize;
use web_sys::Element;
use yew::html::Scope;
use yew::{html, Component, Context, Html, NodeRef, Properties};

use common::mongodb::structs::{
    Comment, ImageExpandState, ImageRequest, PostStats, Resolution, Sort, Source,
};

#[derive(Clone, PartialEq)]
pub struct SortStruct {
    link: Rc<String>,
    text: Rc<String>,
}

#[derive(Clone, PartialEq)]
pub struct SortButtons {
    node_ref: NodeRef,
    sort_current: String,
    sort_current_display: String,
    sort_one: SortStruct,
    sort_two: SortStruct,
}

impl Default for SortStruct {
    fn default() -> Self {
        Self {
            link: Rc::new("/gallery/new".to_string()),
            text: Rc::new("New".to_string()),
        }
    }
}

pub enum SortButtonsMessage {
    CreateButtons,
}

impl SortButtons {
    fn populate_buttons(&mut self, lookup_num: usize) {
        let lookup_buttons = [
            (
                Rc::new("New".to_string()),
                Rc::new("/gallery/new".to_string()),
            ),
            (
                Rc::new("Top".to_string()),
                Rc::new("/gallery/top".to_string()),
            ),
            (
                Rc::new("Views".to_string()),
                Rc::new("/gallery/views".to_string()),
            ),
        ];

        let (text, _) = lookup_buttons.get(lookup_num).unwrap();
        self.sort_current_display = text.to_string();
        let lookup_buttons = lookup_buttons.clone();
        let (text, link) = lookup_buttons.get((lookup_num + 1) % 3).unwrap();

        self.sort_one.link = link.clone();
        self.sort_one.text = text.clone();

        let lookup_buttons = lookup_buttons.clone();
        let (text, link) = lookup_buttons.get((lookup_num + 2) % 3).unwrap();

        self.sort_two.link = link.clone();
        self.sort_two.text = text.clone();
    }
}

// FIXME: This is bad
impl Component for SortButtons {
    type Properties = ();
    type Message = SortButtonsMessage;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            sort_current: String::new(),
            sort_current_display: String::from("New"),
            node_ref: NodeRef::default(),
            sort_one: Default::default(),
            sort_two: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SortButtonsMessage::CreateButtons => {
                match self.sort_current.as_str() {
                    "/new" => {
                        self.populate_buttons(0 as usize);
                    }
                    "/top" => {
                        self.populate_buttons(1 as usize);
                    }
                    "/views" => {
                        self.populate_buttons(2 as usize);
                    }
                    _ => {
                        self.populate_buttons(0 as usize);
                    }
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let current_sort_display = self.sort_current_display.clone();
        let sort_one = self.sort_one.clone();
        let sort_two = self.sort_two.clone();

        html! {
            <>
                <div class="sort-buttons" ref={self.node_ref.clone()}>
                    <button class="dropbtn">{current_sort_display}</button>
                    <div class="sort-button-content">
                        <a href={sort_one.link.to_string()}>{sort_one.text}</a>
                        <a href={sort_two.link.to_string()}>{sort_two.text}</a>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        self.sort_current = self
            .node_ref
            .cast::<Element>()
            .unwrap()
            .base_uri()
            .unwrap()
            .unwrap();

        self.sort_current = self
            .sort_current
            .split_once("gallery")
            .unwrap()
            .1
            .to_string();
        match self.sort_one.eq(&self.sort_two) {
            true => ctx.link().send_message(SortButtonsMessage::CreateButtons),
            false => (),
        }
    }
}

pub enum ImageMessage {
    ToggleExpando(usize),
    QueryImages(Vec<ImageRequest>),
    ShowMore,
    Like(usize),
    None,
}

#[derive(PartialEq, Properties)]
pub struct PostProps {
    pub sort: String,
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
    pub fn toggle_expand(&mut self, avail_width: i32) {
        match &self.state {
            ImageExpandState::Unfocus => {
                let avail_width = avail_width as f32 * 0.71;

                let margin_left = match self.resolution.width > 510 {
                    true => -20,
                    false => 0,
                };
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
            let fetched_images: Vec<ImageRequest> =
                Request::get(format! {"/api/view-posts/1/{}", sort.clone()}.as_str())
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
                let avail_width = ctx
                    .props()
                    .gallery_node_ref
                    .cast::<Element>()
                    .unwrap()
                    .client_width();

                image.toggle_expand(avail_width);
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
            <>
                <SortButtons/>
                <div class={ "images" }>
                    { posts }
                </div>
            </>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        match ctx.props().wheel_position / ctx.props().document_height > 0.8 {
            true => {
                ctx.link().send_message(ImageMessage::ShowMore);
                true
            }
            false => false,
        }
    }
}
