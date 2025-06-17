use evops_types::{EventServiceCreateRequest, EventServiceCreateResponse};

impl crate::AppState {
    pub async fn create_event(
        &self,
        request: EventServiceCreateRequest,
    ) -> EventServiceCreateResponse {
        {
            _ = self.shared_state.db.lock().await;
        }

        todo!();
    }
}
