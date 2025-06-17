use evops_types::{CreateTagError, CreateTagRequest, CreateTagResponse};

impl crate::AppState {
    pub async fn create_tag(
        &self,
        request: CreateTagRequest,
    ) -> Result<CreateTagResponse, CreateTagError> {
        let mut db = self.shared_state.db.lock().await;

        db.create_tag(request).await
    }
}
