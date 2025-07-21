use tonic::{Request, Response, Status};
use uuid::Uuid;

use evops_models::{ApiError, EventDescription};

use crate::pb::tag_service_server::{TagService, TagServiceServer};
use crate::pb::{
    TagServiceCreateRequest, TagServiceCreateResponse, TagServiceDeleteRequest,
    TagServiceDeleteResponse, TagServiceFindRequest, TagServiceFindResponse, TagServiceListRequest,
    TagServiceListResponse, TagServiceSuggestRequest, TagServiceSuggestResponse,
};

pub fn server(state: crate::AppState) -> TagServiceServer<self::Service> {
    TagServiceServer::new(self::Service { state })
}

pub struct Service {
    state: crate::AppState,
}

#[tonic::async_trait]
impl TagService for self::Service {
    async fn list(
        &self,
        request: Request<TagServiceListRequest>,
    ) -> Result<Response<TagServiceListResponse>, Status> {
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
        request: Request<TagServiceCreateRequest>,
    ) -> Result<Response<TagServiceCreateResponse>, Status> {
        let user_id = crate::auth(&self.state, &request)?;
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
        let tag_id = self.state.create_tag(form, user_id).await?;

        let response_data = TagServiceCreateResponse {
            tag_id: tag_id.to_string(),
        };
        Ok(Response::new(response_data))
    }

    async fn find(
        &self,
        request: Request<TagServiceFindRequest>,
    ) -> Result<Response<TagServiceFindResponse>, Status> {
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

    async fn delete(
        &self,
        request: Request<TagServiceDeleteRequest>,
    ) -> Result<Response<TagServiceDeleteResponse>, Status> {
        let user_id = crate::auth(&self.state, &request)?;
        let request_data = request.into_inner();

        let tag_id = evops_models::TagId::new({
            request_data
                .tag_id
                .parse::<Uuid>()
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        });
        self.state.delete_tag(tag_id, user_id).await?;

        Ok(Response::new(TagServiceDeleteResponse {}))
    }

    async fn suggest(
        &self,
        request: Request<TagServiceSuggestRequest>,
    ) -> Result<Response<TagServiceSuggestResponse>, Status> {
        crate::auth(&self.state, &request)?;

        let request_data = request.into_inner();
        let description = {
            EventDescription::try_new(request_data.description)
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        };
        let tags = self.state.suggest_tags(description).await?;
        let response_data = TagServiceSuggestResponse {
            tags: tags.into_iter().map(Into::into).collect(),
        };
        Ok(Response::new(response_data))
    }
}
