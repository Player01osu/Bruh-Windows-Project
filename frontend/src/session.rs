use std::{collections::HashMap};

use common::mongodb::structs::ImageStates;
use gloo_utils::window;
use reqwasm::http::Request;
use serde::Deserialize;
use yew::{html::Scope, Callback};

use crate::{App, AppMsg, pages::gallery::GalleryMsg};

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
            AppMsg(session)
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
                self.image_states_map.insert(state.id.to_string(), state.clone());
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
}
