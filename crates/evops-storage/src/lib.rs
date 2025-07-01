use bon::bon;
use minio::s3::types::S3Api as _;
use tracing::debug;
use url::Url;

mod logic;

pub struct Storage {
    client: minio::s3::Client,
}

impl self::Storage {
    const BUCKET_EVENT_IMAGES: &str = "event-images";
}

#[bon]
impl self::Storage {
    #[builder]
    pub async fn new(base_url: &Url, username: &str, password: &str) -> eyre::Result<Self> {
        let static_provider = minio::s3::creds::StaticProvider::new(username, password, None);
        let client = {
            minio::s3::ClientBuilder::new(base_url.as_str().parse()?)
                .provider(Some(Box::new(static_provider)))
                .build()?
        };

        let this = Self { client };
        let _check_connection = this.client.list_buckets().send().await?;

        this.init_buckets().await?;

        Ok(this)
    }

    async fn init_buckets(&self) -> Result<(), minio::s3::error::Error> {
        let response = {
            self.client
                .bucket_exists(Self::BUCKET_EVENT_IMAGES)
                .send()
                .await?
        };
        if response.exists {
            debug!("bucket {} already exists", Self::BUCKET_EVENT_IMAGES);
        } else {
            debug!("creating bucket {}...", Self::BUCKET_EVENT_IMAGES);
            self.client
                .create_bucket(Self::BUCKET_EVENT_IMAGES)
                .send()
                .await?;
        }

        Ok(())
    }
}
