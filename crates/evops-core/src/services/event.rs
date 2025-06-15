use evops_types::{CreateEventRequest, CreateEventResponse};

impl crate::AppState {
    #[must_use]
    pub async fn create_event(&self, mut request: CreateEventRequest) -> CreateEventResponse {
        {
            _ = self.shared_state.db.lock().await;
        }

        todo!();
    }
}
