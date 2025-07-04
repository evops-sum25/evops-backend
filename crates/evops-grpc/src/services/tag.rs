use tonic::{Request, Response, Status};
use uuid::Uuid;

use evops_models::ApiError;

use crate::pb::tag_service_server::{TagService, TagServiceServer};
use crate::pb::{TagServiceCreateResponse, TagServiceFindResponse, TagServiceListResponse};

pub fn server(state: crate::AppState) -> TagServiceServer<self::Service> {
    TagServiceServer::new(self::Service { state })
}

pub struct Service {
    state: crate::AppState,
}

#[tonic::async_trait]
impl TagService for self::Service {
    async fn find(
        &self,
        request: Request<crate::pb::TagServiceFindRequest>,
    ) -> Result<Response<crate::pb::TagServiceFindResponse>, Status> {
        let request_data = request.into_inner();

        let id = evops_models::TagId::new({
            request_data
                .id
                .parse::<Uuid>()
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        });
        let found_tag = self.state.find_tag(id).await?;

        let response_data = TagServiceFindResponse {
            tag: Some(found_tag.into()),
        };
        Ok(Response::new(response_data))
    }

    async fn list(
        &self,
        request: Request<crate::pb::TagServiceListRequest>,
    ) -> Result<Response<crate::pb::TagServiceListResponse>, Status> {
        let request_data = request.into_inner();

        let last_id = match request_data.last_id {
            Some(id) => Some(evops_models::TagId::new({
                id.parse::<Uuid>()
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            })),
            None => None,
        };
        let limit = match request_data.limit {
            Some(lim) => Some({
                evops_models::PgLimit::try_new(lim)
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            }),
            None => None,
        };
        let tags = self.state.list_tags(last_id, limit).await?;

        let response_data = TagServiceListResponse {
            tags: tags.into_iter().map(Into::into).collect(),
        };
        Ok(Response::new(response_data))
    }

    async fn create(
        &self,
        request: Request<crate::pb::TagServiceCreateRequest>,
    ) -> Result<Response<crate::pb::TagServiceCreateResponse>, Status> {
        let request_data = request.into_inner();

        let form = {
            request_data
                .form
                .ok_or({
                    let err_msg = "TagServiceCreateRequest.form must not be null.";
                    ApiError::InvalidArgument(err_msg.to_owned())
                })?
                .try_into()?
        };
        let tag_id = self.state.create_tag(form).await?;

        let response_data = TagServiceCreateResponse {
            tag_id: tag_id.to_string(),
        };
        Ok(Response::new(response_data))
    }
}
