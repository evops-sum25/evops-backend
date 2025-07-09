use tonic::Request;

use evops_models::{ApiError, ApiResult};

use crate::pb::MlServiceGetTagsRequest;

impl crate::MlClient {
    pub async fn predict_tags(
        &mut self,
        description: evops_models::EventDescription,
    ) -> ApiResult<Vec<evops_models::TagId>> {
        let request_data = MlServiceGetTagsRequest {
            description: description.into_inner(),
        };
        let response = self.grpc.get_tags(Request::new(request_data)).await?;
        let tags = {
            response
                .into_inner()
                .tag_ids
                .into_iter()
                .map(|tag_id| {
                    tag_id
                        .parse()
                        .map(evops_models::TagId::new)
                        .map_err(|e| ApiError::Other(e.to_string()))
                })
                .collect::<Result<Vec<_>, _>>()?
        };
        Ok(tags)
    }
}
