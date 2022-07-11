use reqwasm::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::{html, Component, Context, Html};

#[derive(Properties, Clone, PartialEq)]
struct Image {
    title: String,
    author: String,
    path: String,
    width: String,
    height: String,
}

#[derive(Properties, PartialEq)]
struct ImageListProps {
    images: Vec<Image>,
    on_click: Callback<Image>,
}

#[derive(Properties, PartialEq, Clone)]
struct ImageDetailsProps {
    image: Image,
}

#[function_component(ImageDetails)]
fn image_details(ImageDetailsProps { image }: &ImageDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ image.title.clone() }</h3>
            <img
                src={format!("{}", image.path)}
                alt={format!("{}", image.title)}
                width="300"
                height="300"
            />
        </div>
    }
}

#[function_component(ImageList)]
fn image_list(ImageListProps { images, on_click }: &ImageListProps) -> Html {
    let on_click = on_click.clone();
    images
        .iter()
        .map(|image| {
            let on_image_select = {
                image.width = "300".to_string();
                image.height = "300".to_string();
                let on_click = on_click.clone();
                let image = image.clone();
                Callback::from(move |_| {
                    //image.width = "300".to_string();
                    //image.height = "300".to_string();
                    on_click.emit(image.clone())
                })
            };

            html! {
                <img
                    onclick={on_image_select}
                    src={format!("{}", image.path)}
                    alt={format!("{}", image.title)}
                    width={format!("{}", image.width)}
                    height={format!("{}", image.height)}
                />
            }
        })
        .collect()
}

impl Component for Image {
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Expand => {
                self.width = "400";
                self.height = "400";
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {

        let images = vec![
            Image {
                title: "Yuri mmm I love ".to_string(),
                author: "ur mom ".to_string(),
                path: "assets/posts/test.jpg ".to_string(),
                width: "100".to_string(),
                height: "100".to_string(),
            },
            Image {
                title: "Ay".to_string(),
                author: "bro".to_string(),
                path: "assets/img/blah.jpg ".to_string(),
                width: "100".to_string(),
                height: "100".to_string(),
            },
        ];

        let selected_image = use_state(|| None);

        let on_image_select = {
            let selected_image = selected_image.clone();
            Callback::from(move |image: Image| selected_image.set(Some(image)))
        };

        let details = selected_image.as_ref().map(|image| {
            html! {
                <ImageDetails image={image.clone()} />
            }
        });

        html! {
            <>
                <div>
                    <h1>{ "ell" }</h1>

                    <ImageList images={images} on_click={on_image_select.clone()}/>
                </div>
                { for details }
            </>
        }
    }
}

//#[function_component(App)]
//fn app() -> Html {
//    let images = vec![
//        Image {
//            title: "Yuri mmm I love ".to_string(),
//            author: "ur mom ".to_string(),
//            path: "assets/posts/test.jpg ".to_string(),
//            width: "100".to_string(),
//            height: "100".to_string(),
//        },
//        Image {
//            title: "Ay".to_string(),
//            author: "bro".to_string(),
//            path: "assets/img/blah.jpg ".to_string(),
//            width: "100".to_string(),
//            height: "100".to_string(),
//        },
//    ];
//
//    let selected_image = use_state(|| None);
//
//    let on_image_select = {
//        let selected_image = selected_image.clone();
//        Callback::from(move |image: Image| selected_image.set(Some(image)))
//    };
//
//    let details = selected_image.as_ref().map(|image| {
//        html! {
//            <ImageDetails image={image.clone()} />
//        }
//    });
//
//    html! {
//        <>
//            <div>
//                <h1>{ "ell" }</h1>
//
//                <ImageList images={images} on_click={on_image_select.clone()}/>
//            </div>
//            { for details }
//        </>
//    }
//}

fn main() {
    yew::start_app::<App>();
}
