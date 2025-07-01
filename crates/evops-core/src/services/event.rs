use bytes::Bytes;
use evops_models::{
    ApiError, ApiResult, Event, EventId, EventImage, EventImageId, NewEventForm, PgLimit,
};

impl crate::AppState {
    pub async fn list_events(
        &self,
        last_id: Option<EventId>,
        limit: Option<PgLimit>,
    ) -> ApiResult<Vec<Event>> {
        let events = {
            let mut db = self.shared_state.db.lock().await;
            db.list_events(last_id, limit).await
        }?;
        Ok(events)
    }

    pub async fn create_event(&self, form: NewEventForm) -> ApiResult<EventId> {
        let event_id = {
            let mut db = self.shared_state.db.lock().await;
            db.create_event(form).await
        }?;
        Ok(event_id)
    }

    pub async fn find_event(&self, id: EventId) -> ApiResult<Event> {
        let event = {
            let mut db = self.shared_state.db.lock().await;
            db.find_event(id).await
        }?;
        Ok(event)
    }

    pub async fn push_event_image(
        &self,
        event_id: EventId,
        image: EventImage,
    ) -> ApiResult<EventImageId> {
        let image_id = {
            let mut db = self.shared_state.db.lock().await;
            db.reserve_image(event_id).await?
        };
        let storage = &self.shared_state.storage;
        let upload_image_result = storage.upload_event_image(image_id, image).await;
        if let Err(e) = upload_image_result {
            // TODO: remove from the DB.
            return Err(ApiError::Storage(e.to_string()));
        }

        Ok(image_id)
    }

    pub async fn find_event_image(&self, _id: EventImageId) -> ApiResult<Bytes> {
        todo!()
    }
}
