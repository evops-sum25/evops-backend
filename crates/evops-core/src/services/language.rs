use evops_models::{LanguageId, LanguageName};

impl crate::AppState {
    pub async fn add_language(&self, name: LanguageName) -> ApiResult<LanguageId> {
        todo!();
    }
}
