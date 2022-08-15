use mongodb::{Client, Collection, Database};

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct MongodbDatabase {
    pub database: Database,
}

pub enum CollectionList {
    Posts,
    Comments,
}

/// Abstracted database with fixed collections
impl MongodbDatabase {
    pub async fn mongo_connect() -> Self {
        let uri =
            std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

        let client = Client::with_uri_str(uri).await.expect("failed to connect");
        let database = client.database("yuri-web-server");
        Self { database }
    }

    /// Returns a collection handle.
    /// If generic is specified, the generic
    /// struct must impliment serialize and
    /// deserialize.
    pub fn collection<'a, T>(&self, collection: CollectionList) -> Collection<T>
    where
        T: Serialize + Deserialize<'a>,
    {
        match collection {
            CollectionList::Posts => self.database.collection::<T>("yuriPosts"),
            CollectionList::Comments => self.database.collection::<T>("yuriComments"),
        }
    }
}
