use mongodb::{ 
	bson::{Document, doc},
	Client,
	Collection 
};
use std::env;

mod configs;
mod strings;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let uri = match env::var(configs::MONGODB_CONN_STR_ENV) {
        Ok(uri) => {
            uri
        }, 
        _ => {
            panic!("{}", strings::GET_MONGODB_CONN_STR_ENV_FAIL);
        }
    };
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await?;
    // Get a handle on the movies collection
    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");
    // Find a movie based on the title value
    let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }).await?;
    // Print the document
    println!("Found a movie:\n{:#?}", my_movie);
    Ok(())
}
