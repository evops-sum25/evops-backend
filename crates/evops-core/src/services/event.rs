use evops_models::{CreateEventError, EventServiceCreateRequest, EventServiceCreateResponse};

impl crate::AppState {
    pub async fn create_event(
        &self,
        request: EventServiceCreateRequest,
    ) -> Result<EventServiceCreateResponse, CreateEventError> {
        let mut db = self.shared_state.db.lock().await;

        Ok(EventServiceCreateResponse {
            event: db.create_event(request.form).await?,
        })
    }
}
