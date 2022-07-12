use reqwasm::http::{Request};
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
    pub width: usize,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Id {
    #[serde(rename = "$oid")]
    oid: String
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
                self.width = 1300;
                self.state = ImageExpandState::Focus
            }
            ImageExpandState::Focus => {
                self.width = 550;
                self.state = ImageExpandState::Unfocus
            }
        }
    }
}

impl App {
    pub fn view_images(&self, image_id: usize, image: &Image, link: &Scope<Self>) -> Html {
        html! {
            <div>
                <img alt={format!("{} {}", image.author, image.title)}
                    src={format!("{}", image.path)}
                    width={format!("{}", image.width)}
                    loading="lazy"
                    onclick={link.callback(move |_| Msg::ToggleExpando(image_id))}/>
            </div>
        }
    }
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(
            async {
                let fetched_images: Vec<ImageRequest> = Request::get("/api/gallery_display")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                Msg::QueryImages(fetched_images)
            }
        );

        //let width = 550;
        //let images = vec![
        //    Image {
        //        state: ImageExpandState::Unfocus,
        //        title: "Yuri mmm I love ".to_string(),
        //        author: "ur mom ".to_string(),
        //        time: 21,
        //        path: "assets/posts/test.jpg ".to_string(),
        //        width,
        //    },
        //    Image {
        //        state: ImageExpandState::Unfocus,
        //        title: "Ay".to_string(),
        //        author: "bro".to_string(),
        //        time: 2001,
        //        path: "assets/img/blah.jpg ".to_string(),
        //        width,
        //    },
        //    Image {
        //        state: ImageExpandState::Unfocus,
        //        title: "frog".to_string(),
        //        author: "crazzzy".to_string(),
        //        time: 22,
        //        path: "assets/posts/FB_IMG_1634517925950.png ".to_string(),
        //        width,
        //    },
        //];

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
                            width: 550,
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
                <div>
                    <div class={ "header-all" }>
                        <div class={ "header" }>
                            <h1>{ "Wholesome Yuri" }</h1>
                        </div>
                    </div>

                    <div class={ "container" }>
                        { posts }
                    </div>
                </div>
            </>
        }
    }
}
fn main() {
        println!("hi");
    yew::start_app::<App>();
}
