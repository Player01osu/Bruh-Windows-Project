use futures::Future;
use gloo_utils::window;
use reqwasm::http::Request;
use serde::Deserialize;
use yew::html::Scope;
//use futures::j

use crate::{App, AppMsg};

#[derive(Deserialize)]
pub struct Session {
    #[serde(rename = "public")]
    pub user_pub: String,
    #[serde(rename = "private", default = "String::default")]
    pub user_priv: String,
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
}
