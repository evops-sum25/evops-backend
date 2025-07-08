use tokio::sync::Mutex;
use tonic::Request;
use tonic::transport::Channel;
use url::Url;
use uuid::Uuid;

use evops_models::{ApiError, ApiResult, EventDescription, TagId};

use crate::pb::MlServiceGetTagsRequest;
use crate::pb::ml_service_client::MlServiceClient;

pub mod pb;

pub struct MlClient {
    client: Mutex<MlServiceClient<Channel>>,
}

impl MlClient {
    pub async fn new(server_url: &Url) -> ApiResult<Self> {
        let channel = Channel::from_shared(server_url.to_string())
            .map_err(|e| ApiError::InvalidArgument(format!("Invalid server URL: {}", e)))?
            .connect()
            .await?;

        Ok(Self {
            client: Mutex::new(MlServiceClient::new(channel)),
        })
    }

    pub async fn get_tags(&self, description: EventDescription) -> ApiResult<Vec<TagId>> {
        let request = Request::new(MlServiceGetTagsRequest {
            description: description.into_inner(),
        });

        let mut client = self.client.lock().await;
        let response_raw = client.get_tags(request).await.map_err(ApiError::from)?;
        let response = {
            response_raw
                .into_inner()
                .tag_ids
                .into_iter()
                .filter_map(|tag| {
                    Uuid::parse_str(&tag)
                        .map(TagId::new)
                        .map_err(|e| {
                            tracing::warn!("Ml client: Skipping invalid tag UUID: {}", tag);
                            e
                        })
                        .ok()
                })
                .collect()
        };
        Ok(response)
    }
}
