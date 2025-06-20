use tonic::transport::{Channel, Endpoint};

use evops_pb::api::event_service_client::EventServiceClient;
use evops_pb::api::tag_service_client::TagServiceClient;
use evops_pb::api::user_service_client::UserServiceClient;

pub struct Api {
    pub event_service: EventServiceClient<Channel>,
    pub tag_service: TagServiceClient<Channel>,
    pub user_service: UserServiceClient<Channel>,
}

impl Api {
    pub async fn try_connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        let endpoint = Endpoint::new(dst)?;

        Ok(Self {
            event_service: EventServiceClient::connect(endpoint.clone()).await?,
            tag_service: TagServiceClient::connect(endpoint.clone()).await?,
            user_service: UserServiceClient::connect(endpoint).await?,
        })
    }
}
