use evops_models::{
    CreateTagError, FindTagError, ListTagsError, TagServiceCreateRequest, TagServiceCreateResponse,
    TagServiceFindRequest, TagServiceFindResponse, TagServiceListRequest, TagServiceListResponse,
};

impl crate::AppState {
    pub async fn find_tag(
        &self,
        request: TagServiceFindRequest,
    ) -> Result<TagServiceFindResponse, FindTagError> {
        let mut db = self.shared_state.db.lock().await;

        Ok(TagServiceFindResponse {
            tag: db.find_tag(request.id).await?,
        })
    }

    pub async fn list_tags(
        &self,
        _request: TagServiceListRequest,
    ) -> Result<TagServiceListResponse, ListTagsError> {
        let mut db = self.shared_state.db.lock().await;

        Ok(TagServiceListResponse {
            tags: db.list_tags().await?,
        })
    }

    pub async fn create_tag(
        &self,
        request: TagServiceCreateRequest,
    ) -> Result<TagServiceCreateResponse, CreateTagError> {
        let mut db = self.shared_state.db.lock().await;

        Ok(TagServiceCreateResponse {
            tag: db.create_tag(request.form).await?,
        })
    }
}
