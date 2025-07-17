use evops_models::{ApiResult, LanguageId, NewLanguageForm};

impl crate::AppState {
    #[allow(clippy::unused_async)]
    pub async fn add_language(&self, _form: NewLanguageForm) -> ApiResult<LanguageId> {
        todo!();
    }
}
