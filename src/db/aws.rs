use dotenv::var;
use s3::{creds::Credentials, Bucket};

#[derive(Debug, Clone)]
pub struct S3 {
    bucket: Bucket,
}

impl S3 {
    pub fn init() -> Self {
        let bucket_name = var("BUCKET_NAME").expect("Cannot read the BUCKET_NAME in the env");
        let region = var("REGION").expect("Cannot read the REGION in the env");
        let credentials = Credentials::from_env().expect("Cannot create AWS S3 credentials");

        let bucket = Bucket::new(
            &bucket_name,
            region
                .parse()
                .expect("Cannot use the REGION value in the env to parse as region"),
            credentials,
        )
        .expect("Cannot map the internal AWS S3 bucket to the bucket online");

        Self { bucket }
    }

    pub fn get_storage(&self) -> Bucket {
        self.bucket.clone()
    }
}
