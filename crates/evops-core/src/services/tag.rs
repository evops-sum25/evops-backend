use evops_models::{ApiResult, NewTagForm, PgLimit, Tag, TagId};

impl crate::AppState {
    pub async fn list_tags(
        &self,
        last_id: Option<TagId>,
        limit: Option<PgLimit>,
    ) -> ApiResult<Vec<Tag>> {
        let tags = {
            let mut db = self.shared_state.db.lock().await;
            db.list_tags(last_id, limit).await
        }?;
        Ok(tags)
    }

    pub async fn create_tag(&self, request: NewTagForm) -> ApiResult<TagId> {
        let tag_id = {
            let mut db = self.shared_state.db.lock().await;
            db.create_tag(request).await
        }?;
        Ok(tag_id)
    }

    pub async fn find_tag(&self, id: TagId) -> ApiResult<Tag> {
        let tag = {
            let mut db = self.shared_state.db.lock().await;
            db.find_tag(id).await
        }?;
        Ok(tag)
    }
}
