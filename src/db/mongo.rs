use dotenv::var;
use mongodb::{options::ClientOptions, Client, Collection};

pub struct Mongo {
    client: Client,
}

impl Mongo {
    pub async fn init() -> Self {
        let mongo_uri = var("MONGODB_URI").expect("Cannot read mongodb uri from env");
        let mut client_options = ClientOptions::parse(mongo_uri)
            .await
            .expect("Cannot parse MONGODB_URI");
        client_options.app_name = Some("simple-file-sharing".into());
        Self {
            client: Client::with_options(client_options).expect("Cannot create the MongoDB client"),
        }
    }

    pub fn get_collection<T>(&self, collection_name: &str) -> Collection<T> {
        self.client
            .database("simple-file-sharing")
            .collection(collection_name)
    }
}
