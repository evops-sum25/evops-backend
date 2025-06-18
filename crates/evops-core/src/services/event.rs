use evops_models::{
    CreateEventError, EventServiceCreateRequest, EventServiceCreateResponse,
    EventServiceFindRequest, EventServiceFindResponse, EventServiceListRequest,
    EventServiceListResponse, FindEventError, ListEventsError,
};

impl crate::AppState {
    pub async fn find_event(
        &self,
        request: EventServiceFindRequest,
    ) -> Result<EventServiceFindResponse, FindEventError> {
        todo!()
    }

    pub async fn list_events(
        &self,
        request: EventServiceListRequest,
    ) -> Result<EventServiceListResponse, ListEventsError> {
        todo!()
    }

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
