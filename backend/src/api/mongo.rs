use actix_web::web::Data;
use futures::TryStreamExt;
use mongodb::{Client, options::{ ClientOptions, FindOptions } };
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct YuriPosts {
    pub title: String,
    pub op: String,
    pub author: String,
    pub path: String,
    pub tags: Vec<String>,
    pub time: u64,
}
//impl<'a> Borrow<Borrowed + 'a> for YuriPosts {
//    fn borrow(&self) -> &(Borrowed + 'a) {
//        self
//    }
//}

// TODO: Implement some sort of way to connect to other collections
pub enum MongodbCollection {
    Yuriposts(mongodb::Collection<YuriPosts>),
}

pub enum WebServerdb {
    YuriPosts,
}

pub struct MongodbDatabase {
    pub collection: Data<mongodb::Collection<YuriPosts>>,
}

impl MongodbDatabase {
    pub fn new(collection: Data<mongodb::Collection<YuriPosts>>) -> MongodbDatabase {
        let mongodb_collection = MongodbDatabase {
            collection,
        };
        mongodb_collection
    }

    /// Generates a cursor for the collection, iterating through it and
    /// pushing its results to the vector for n amount of items.
    pub async fn find(&self,
        filter: Document,
        find_options: Option<FindOptions>,
        amount: u16) -> Vec<YuriPosts>{
        let mut number: u16 = 0;
        let mut cursor = self
            .collection
            .find(filter, find_options)
            .await
            .expect("Failed to generate find cursor");
        let mut paths: Vec<YuriPosts> = Vec::new();

        while let Some(yuri_posts) = cursor.try_next().await.expect("Failed to iterate through cursor") {
            println!("path: {}", yuri_posts.path);
            paths.push(yuri_posts);
            number += 1;
            if number > amount {
                break;
            }
        }
        paths
    }

    /// Generates a cursor for the collection, iterating through it and
    /// return one item.
    pub async fn find_one(&self,
        filter: Document,
        find_options: Option<FindOptions>) -> YuriPosts{
        let mut cursor = self
            .collection
            .find(filter, find_options)
            .await
            .expect("Failed to generate find cursor");

        return cursor.try_next().await.expect("Failed to iterate through cursor").expect("Failed to unwrap");
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

}
