use evops_models::{
    ApiResult, TagServiceCreateRequest, TagServiceCreateResponse, TagServiceFindRequest,
    TagServiceFindResponse, TagServiceListRequest, TagServiceListResponse,
};

impl crate::AppState {
    pub async fn find_tag(
        &self,
        request: TagServiceFindRequest,
    ) -> ApiResult<TagServiceFindResponse> {
        let mut db = self.shared_state.db.lock().await;

        Ok(TagServiceFindResponse {
            tag: db.find_tag(request.id).await?,
        })
    }

    pub async fn list_tags(
        &self,
        request: TagServiceListRequest,
    ) -> ApiResult<TagServiceListResponse> {
        let mut db = self.shared_state.db.lock().await;
        let tags = db.list_tags(request.last_id, request.limit).await?;
        Ok(TagServiceListResponse { tags })
    }

    pub async fn create_tag(
        &self,
        request: TagServiceCreateRequest,
    ) -> ApiResult<TagServiceCreateResponse> {
        let mut db = self.shared_state.db.lock().await;

        Ok(TagServiceCreateResponse {
            tag: db.create_tag(request.form).await?,
        })
    }
}
