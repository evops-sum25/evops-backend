use evops_models::{
    ApiResult, EventServiceCreateRequest, EventServiceCreateResponse, EventServiceFindRequest,
    EventServiceFindResponse, EventServiceListRequest, EventServiceListResponse,
};

impl crate::AppState {
    pub async fn find_event(
        &self,
        request: EventServiceFindRequest,
    ) -> ApiResult<EventServiceFindResponse> {
        let mut db = self.shared_state.db.lock().await;

        Ok(EventServiceFindResponse {
            event: db.find_event(request.id).await?,
        })
    }

    pub async fn list_events(
        &self,
        request: EventServiceListRequest,
    ) -> ApiResult<EventServiceListResponse> {
        let mut db = self.shared_state.db.lock().await;
        let (events, last_id) = db.list_events(request.last_id, request.limit).await?;
        Ok(EventServiceListResponse { events, last_id })
    }

    pub async fn create_event(
        &self,
        request: EventServiceCreateRequest,
    ) -> ApiResult<EventServiceCreateResponse> {
        let mut db = self.shared_state.db.lock().await;

        Ok(EventServiceCreateResponse {
            event: db.create_event(request.form).await?,
        })
    }
}
