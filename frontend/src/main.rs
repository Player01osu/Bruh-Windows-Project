use reqwasm::http::{Headers, Request};
use serde::{Deserialize, Serialize};
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
    pub width: String,
    pub height: String,
}

impl Image {
    pub fn toggle_expand(&mut self) {
        match &self.state {
            ImageExpandState::Focus => {
                self.height = "300".to_string();
                self.width = "300".to_string();
                self.state = ImageExpandState::Unfocus
            }
            ImageExpandState::Unfocus => {
                self.height = "100".to_string();
                self.width = "100".to_string();
                self.state = ImageExpandState::Focus
            }
        }
    }
}

impl App {
    pub fn view_images(&self, image_id: usize, image: &Image, link: &Scope<Self>) -> Html {

        html! {
            <div>
                <img src={format!("{}", image.path)}
                    width={format!("{}", image.width)}
                    height={format!("{}", image.height)}
                    onclick={link.callback(move |_| Msg::ToggleExpando(image_id))}/>
            </div>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let images = vec![
            Image {
                state: ImageExpandState::Unfocus,
                title: "Yuri mmm I love ".to_string(),
                author: "ur mom ".to_string(),
                time: 21,
                path: "test.jpg ".to_string(),
                width: "100".to_string(),
                height: "100".to_string(),
            },
            Image {
                state: ImageExpandState::Unfocus,
                title: "Ay".to_string(),
                author: "bro".to_string(),
                time: 2001,
                path: "assets/img/blah.jpg ".to_string(),
                width: "100".to_string(),
                height: "100".to_string(),
            },
        ];

        Self { images }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleExpando(image_id) => {
                let image = self.images.get_mut(image_id).unwrap();

                image.toggle_expand();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let posts = self.images.chunks(21).enumerate().map(|(id, images)| {
            let image_list = images
                .iter()
                .enumerate()
                .map(|(id, image)| self.view_images(id, image, ctx.link()));
            html! {
                <div>
                    {for image_list}
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
