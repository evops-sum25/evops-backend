use std::time::Duration;

use url::Url;

use crate::pb::ml_service_client::MlServiceClient;

mod logic;
pub mod pb;

pub struct MlClient {
    grpc: MlServiceClient<tonic::transport::Channel>,
}

impl MlClient {
    pub async fn new(server_url: &Url) -> eyre::Result<Self> {
        let channel;
        // HACK: this should be done in Docker.
        loop {
            tracing::debug!("connecting to ml server, please wait...");
            let connection_result = {
                tonic::transport::Channel::from_shared(server_url.to_string())?
                    .connect()
                    .await
            };
            if let Ok(ch) = connection_result {
                channel = ch;
                break;
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
        let ml_grpc_client = MlServiceClient::new(channel);
        Ok(Self {
            grpc: ml_grpc_client,
        })
    }
}
