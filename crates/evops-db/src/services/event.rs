use evops_types::{CreateEventError, Event, EventForm};

impl crate::Database {
    pub async fn create_event(&mut self, request: EventForm) -> Result<Event, CreateEventError> {
        todo!();
    }
}
