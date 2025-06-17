use evops_types::{CreateEventError, CreateEventRequest, CreateEventResponse};

impl crate::Database {
    pub async fn create_event(
        &mut self,
        request: CreateEventRequest,
    ) -> Result<CreateEventResponse, CreateEventError> {
        todo!();
    }
}
