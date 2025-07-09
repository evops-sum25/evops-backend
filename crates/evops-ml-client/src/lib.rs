use url::Url;

use crate::pb::ml_service_client::MlServiceClient;

mod logic;
pub mod pb;

pub struct MlClient {
    grpc: MlServiceClient<tonic::transport::Channel>,
}

impl MlClient {
    pub async fn new(server_url: &Url) -> eyre::Result<Self> {
        let channel = {
            tonic::transport::Channel::from_shared(server_url.to_string())?
                .connect()
                .await?
        };
        let ml_grpc_client = MlServiceClient::new(channel);
        Ok(Self {
            grpc: ml_grpc_client,
        })
    }
}
