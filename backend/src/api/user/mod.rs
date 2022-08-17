pub mod generate_user;
pub mod get_user;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use common::mongodb::structs::UsersDb;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
struct UserId {
    user_pub: String,
    user_priv: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Users {
    private: String,
    pub public: String,
}

impl Users {
    pub fn new() -> Self {
        let private = Uuid::new_v4().to_string();
        let public = Self::hash_priv(&private);
        Self { private, public }
    }

    pub fn hash_priv(private: &String) -> String {
        let mut public = DefaultHasher::new();
        public.write(private.as_bytes());
        private.to_string().hash(&mut public);
        public.finish().to_string()
    }
}
