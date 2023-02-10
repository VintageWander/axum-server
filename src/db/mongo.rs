use crate::Result;
use dotenv::var;
use mongodb::{options::ClientOptions, Client, Collection};

pub struct Mongo {
    client: Client,
}

impl Mongo {
    pub async fn init() -> Result<Self> {
        let mongo_uri = var("MONGODB_URI").expect("Cannot read mongodb uri from env");
        let mut client_options = ClientOptions::parse(mongo_uri).await?;
        client_options.app_name = Some("simple-file-sharing".into());
        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    pub fn get_collection<T>(&self, collection_name: &str) -> Collection<T> {
        self.client
            .database("simple-file-sharing")
            .collection(collection_name)
    }
}
