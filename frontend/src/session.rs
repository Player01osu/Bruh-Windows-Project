use gloo_utils::window;
use reqwasm::http::Request;
use serde::Deserialize;
use yew::html::Scope;

use crate::{App, AppMsg};

#[derive(Deserialize)]
pub struct Session {
    #[serde(rename = "_id")]
    user_id: Oid,
}

#[derive(Deserialize)]
pub struct Oid {
    #[serde(rename = "$oid")]
    oid: String,
}

impl Session {
    pub fn init(link: &Scope<App>) {
        let user_id = window()
            .local_storage()
            .unwrap()
            .expect("Failed to get local storage")
            .get("user_id")
            .unwrap();

        link.send_future(async move {
            let session: Session = match user_id {
                Some(session) => Request::get(format! {"/api/user/get_user/{}", session}.as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap(),
                None => Request::put(format! {"/api/user/generate_user"}.as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap(),
            };
            AppMsg(session.user_id.oid)
        });
    }
}
