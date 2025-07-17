use evops_models::{ApiResult, LanguageId, NewLanguageForm};

impl crate::AppState {
    pub async fn add_language(&self, _form: NewLanguageForm) -> ApiResult<LanguageId> {
        todo!();
    }
}
