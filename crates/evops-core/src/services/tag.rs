use evops_models::{
    ApiResult, TagServiceCreateRequest, TagServiceCreateResponse, TagServiceFindRequest,
    TagServiceFindResponse, TagServiceListRequest, TagServiceListResponse,
};

impl crate::AppState {
    pub async fn find_tag(
        &self,
        request: TagServiceFindRequest,
    ) -> ApiResult<TagServiceFindResponse> {
        let tag = {
            let mut db = self.shared_state.db.lock().await;
            db.find_tag(request.id).await
        }?;
        Ok(TagServiceFindResponse { tag })
    }

    pub async fn list_tags(
        &self,
        request: TagServiceListRequest,
    ) -> ApiResult<TagServiceListResponse> {
        let tags = {
            let mut db = self.shared_state.db.lock().await;
            db.list_tags(request.last_id, request.limit).await
        }?;
        Ok(TagServiceListResponse { tags })
    }

    pub async fn create_tag(
        &self,
        request: TagServiceCreateRequest,
    ) -> ApiResult<TagServiceCreateResponse> {
        let tag = {
            let mut db = self.shared_state.db.lock().await;
            db.create_tag(request.form).await
        }?;
        Ok(TagServiceCreateResponse { tag })
    }
}
