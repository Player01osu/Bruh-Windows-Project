//use reqwasm::http::{Headers, Request};
//use serde::{Deserialize, Serialize};
use yew::html::Scope;
use yew::prelude::*;
use yew::{html, Component, Context, Html};

struct App {
    images: Vec<Image>,
}

enum Msg {
    ToggleExpando(usize),
}

#[derive(Clone, PartialEq)]
enum ImageExpandState {
    Unfocus,
    Focus,
}

#[derive(Properties, Clone, PartialEq)]
struct Image {
    pub state: ImageExpandState,
    pub title: String,
    pub author: String,
    pub path: String,
    pub time: usize,
    pub width: usize,
}

impl Image {
    pub fn toggle_expand(&mut self) {
        match &self.state {
            ImageExpandState::Unfocus => {
                self.width = 1200;
                self.state = ImageExpandState::Focus
            }
            ImageExpandState::Focus => {
                self.width = 500;
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

    fn create(_ctx: &Context<Self>) -> Self {
        let width = 500;
        let images = vec![
            Image {
                state: ImageExpandState::Unfocus,
                title: "Yuri mmm I love ".to_string(),
                author: "ur mom ".to_string(),
                time: 21,
                path: "assets/posts/test.jpg ".to_string(),
                width,
            },
            Image {
                state: ImageExpandState::Unfocus,
                title: "Ay".to_string(),
                author: "bro".to_string(),
                time: 2001,
                path: "assets/img/blah.jpg ".to_string(),
                width,
            },
            Image {
                state: ImageExpandState::Unfocus,
                title: "frog".to_string(),
                author: "crazzzy".to_string(),
                time: 22,
                path: "assets/posts/FB_IMG_1634517925950.png ".to_string(),
                width,
            },
        ];

        Self { images }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleExpando(image_id) => {
                let image = self.images.get_mut(image_id).unwrap();

                image.toggle_expand();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let posts = self.images
            .iter()
            .enumerate()
            .map(|(id, image)|{
                let image_list = self.view_images(id, image, ctx.link());
                html! {
                    <div>
                        {image_list}
                    </div>
                }
            });

        html! {
            <>
                <div>
                    <h1>{ "Click on image to expand!" }</h1>
                    { for posts }
                </div>
            </>
        }
    }
}
fn main() {
    yew::start_app::<App>();
}
