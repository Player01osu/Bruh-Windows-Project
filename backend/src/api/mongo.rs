use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct YuriPosts {
    pub path: String,
    pub author: String,
    pub tags: Vec<String>,
}

pub async fn mongo_connect() -> mongodb::Collection<YuriPosts> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.expect("bruh");

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).expect("bruh");

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await.expect("bruh") {
        println!("{} big poo time", db_name);
    }

    // Get a handle to a database.
    let db = client.database("yuri-web-server");

    // Get a handle to a collection in the database.
    let collection = db.collection::<YuriPosts>("yuriPosts");
    collection
}
