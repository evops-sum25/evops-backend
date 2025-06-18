use evops_models::{CreateTagError, TagServiceCreateRequest, TagServiceCreateResponse};

impl crate::AppState {
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
