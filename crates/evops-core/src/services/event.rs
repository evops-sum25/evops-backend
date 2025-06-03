use crate::types::{EventServiceCreateRequest, EventServiceCreateResponse};

#[must_use]
pub async fn create(
    state: &crate::AppState,
    mut request: EventServiceCreateRequest,
) -> EventServiceCreateResponse {
    {
        _ = state.db.lock().await;
    }

    request.name.make_ascii_uppercase();
    EventServiceCreateResponse {
        name: request.name,
        description: request.description.repeat(2),
    }
}
