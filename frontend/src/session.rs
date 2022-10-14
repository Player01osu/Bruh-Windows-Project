use std::collections::HashMap;

use common::mongodb::structs::ImageLiked;
use common::mongodb::structs::ImageStatesDeserialize as ImageStates;
use gloo_utils::window;
use reqwasm::http::Request;
use serde::Deserialize;
use yew::{html::Scope, Callback};

use crate::{pages::gallery::GalleryMsg, App, AppMsg};

#[derive(Deserialize, Default, Clone, PartialEq, Debug)]
pub struct Session {
    #[serde(rename = "public")]
    pub user_pub: String,
    #[serde(rename = "private")]
    pub user_priv: Option<String>,
    #[serde(default)]
    pub image_states: Vec<ImageStates>,
    #[serde(skip)]
    pub image_states_map: HashMap<String, ImageStates>,
    #[serde(skip)]
    pub gallery_callback: Option<Callback<GalleryMsg>>,
    #[serde(skip)]
    pub app_message: Callback<AppMsg>,
}

pub enum State {
    Like(ImageLiked),
    Views,
    Upload,
}

pub struct ChangeState {
    pub state: State,
    pub post_id: String,
}
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

impl Session {
    pub fn init(link: &Scope<App>) {
        let user_priv = window()
            .local_storage()
            .unwrap()
            .expect("Failed to get local storage")
            .get("private")
            .unwrap();

        link.send_future(async move {
            let session: Session = match user_priv {
                Some(private) => {
                    match Request::get(format! {"/api/user/get_user/{}", private}.as_str())
                        .send()
                        .await
                        .unwrap()
                        .json::<Session>()
                        .await
                    {
                        Ok(v) => v,
                        Err(_) => Self::generate_user().await,
                    }
                }
                None => Self::generate_user().await,
            };
            AppMsg::LoadUser(session)
        });
    }

    async fn generate_user() -> Session {
        let local_storage = window().local_storage().unwrap().unwrap();
        local_storage.remove_item("public").unwrap();
        local_storage.remove_item("private").unwrap();
        Request::put(format! {"/api/user/generate_user"}.as_str())
            .send()
            .await
            .unwrap()
            .json::<Session>()
            .await
            .unwrap()
    }

    pub fn map_image_states(&mut self) {
        if !&self.image_states.is_empty() {
            for state in self.image_states.clone() {
                self.image_states_map
                    .insert(state.id.to_string(), state.clone());
            }
        }
    }

    pub fn update_state(&mut self, returned: Self) {
        let user_priv = window()
            .local_storage()
            .unwrap()
            .expect("Failed to get local storage")
            .get("private")
            .unwrap()
            .expect("Should not be None at this point");

        self.user_priv = Some(user_priv);
        self.image_states = returned.image_states;
        self.map_image_states();
    }

    pub fn change_state(&mut self, state_change: ChangeState) {
        console_log!("{:#?}", &self);
        //let state = self.image_states_map;
        let mut image_state_default = ImageStates {
            id: state_change.post_id.clone(),
            ..Default::default()
        };

        let mut image_state = match self.image_states_map.get_mut(&state_change.post_id) {
            Some(v) => v,
            None => &mut image_state_default,
        };

        match state_change.state {
            State::Like(like_state) => match like_state {
                ImageLiked::Liked => {
                    image_state.like_state = ImageLiked::Liked;
                }
                ImageLiked::Unliked => {
                    image_state.like_state = ImageLiked::Unliked;
                }
            },
            State::Views => todo!(),
            State::Upload => todo!(),
        }
        console_log!("{:#?}", &self);
    }
}
