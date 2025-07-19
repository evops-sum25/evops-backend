use std::pin::Pin;

use bytes::Bytes;
use futures::Stream;

use evops_models::{
    ApiResult, Event, EventId, EventImage, EventImageId, EventImageIds, NewEventForm, PgLimit,
    TagId, UpdateEventForm, UserId,
};
use uuid::Uuid;

impl crate::AppState {
    pub async fn list_events(
        &self,
        last_id: Option<EventId>,
        limit: Option<PgLimit>,
        tags: Vec<TagId>,
        search: Option<String>,
    ) -> ApiResult<Vec<Event>> {
        let search = search.map(|s| s.trim().to_lowercase());
        let events = {
            let mut db = self.shared_state.db.lock().await;
            db.list_events(last_id, limit, tags, search).await
        }?;
        Ok(events)
    }

    pub async fn create_event(&self, form: NewEventForm, author_id: UserId) -> ApiResult<EventId> {
        let event_id = {
            let mut db = self.shared_state.db.lock().await;
            db.create_event(form, author_id).await
        }?;
        Ok(event_id)
    }

    pub async fn find_event(&self, event_id: EventId) -> ApiResult<Event> {
        let event = {
            let mut db = self.shared_state.db.lock().await;
            db.find_event(event_id).await
        }?;
        Ok(event)
    }

    pub async fn delete_event(&self, event_id: EventId, user_id: UserId) -> ApiResult<()> {
        let image_ids = {
            let mut db = self.shared_state.db.lock().await;
            db.delete_event(event_id, user_id).await
        }?;
        for image_id in image_ids.into_inner() {
            _ = self.shared_state.storage.delete_event_image(image_id).await;
        }
        Ok(())
    }

    pub async fn update_event(
        &self,
        event_id: EventId,
        user_id: UserId,
        form: UpdateEventForm,
    ) -> ApiResult<()> {
        let mut db = self.shared_state.db.lock().await;
        db.update_event(event_id, user_id, form).await
    }

    pub async fn push_event_image(
        &self,
        user_id: UserId,
        event_id: EventId,
        image: EventImage,
    ) -> ApiResult<EventImageId> {
        let storage = &self.shared_state.storage;
        let image_id = EventImageId::new(Uuid::now_v7());
        storage.upload_event_image(image_id, image).await?;
        let db_result = {
            let mut db = self.shared_state.db.lock().await;
            db.reserve_image(event_id, image_id, user_id).await
        };
        db_result?;
        Ok(image_id)
    }

    pub async fn reorder_images(
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

    #[allow(clippy::unused_async)]
    pub async fn delete_image(&self, _id: EventImageId) -> ApiResult<()> {
        todo!()
    }
}
