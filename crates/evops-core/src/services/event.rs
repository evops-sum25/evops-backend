use evops_types::{CreateEventRequest, CreateEventResponse};

impl crate::AppState {
    pub async fn create_event(&self, request: CreateEventRequest) -> CreateEventResponse {
        {
            _ = self.shared_state.db.lock().await;
        }

        todo!();
    }
}
