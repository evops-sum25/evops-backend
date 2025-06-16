use evops_types::{CreateTagRequest, CreateTagResponse};

impl crate::AppState {
    #[must_use]
    pub async fn create_tag(
        &self,
        request: CreateTagRequest,
    ) -> Result<CreateTagResponse, crate::errors::CreateTagError> {
        let mut db = self.shared_state.db.lock().await;
        Ok(db.create_tag(request).await?)
    }
}
