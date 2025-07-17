use std::pin::Pin;

use bytes::Bytes;
use futures::Stream;

use evops_models::{
    ApiResult, Event, EventId, EventImage, EventImageId, EventImageIds, NewEventForm, PgLimit,
    UpdateEventForm,
};
use uuid::Uuid;

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

    pub async fn delete_event(&self, id: EventId) -> ApiResult<()> {
        // TODO: delete from MINIO
        let mut db = self.shared_state.db.lock().await;
        db.delete_event(id).await
    }

    pub async fn update_event(&self, form: UpdateEventForm) -> ApiResult<()> {
        let mut db = self.shared_state.db.lock().await;
        db.update_event(form).await
    }

    pub async fn push_event_image(
        &self,
        event_id: EventId,
        image: EventImage,
    ) -> ApiResult<EventImageId> {
        let storage = &self.shared_state.storage;
        let image_id = EventImageId::new(Uuid::now_v7());
        storage.upload_event_image(image_id, image).await?;
        let db_result = {
            let mut db = self.shared_state.db.lock().await;
            db.reserve_image(event_id, image_id).await
        };
        db_result?;
        Ok(image_id)
    }

    pub async fn reorder_image(
        &self,
        event_id: EventId,
        image_order: EventImageIds,
    ) -> ApiResult<()> {
        let mut db = self.shared_state.db.lock().await;
        db.reorder_images(event_id, image_order).await
    }

    pub async fn stream_event_image(
        &self,
        id: EventImageId,
    ) -> ApiResult<Pin<Box<dyn Stream<Item = ApiResult<Bytes>> + Send>>> {
        let storage = &self.shared_state.storage;
        let image_stream = storage.stream_event_image(id).await?;
        Ok(image_stream)
    }
}
