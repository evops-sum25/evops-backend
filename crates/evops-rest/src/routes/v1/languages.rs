use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse;
use crate::types::{LanguageServiceAddRequest, LanguageServiceAddResponse};

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::LanguageService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with(
        "/",
        post_with(self::post, self::post_docs),
        self::route_docs,
    )
}

fn post_docs(mut o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.LanguageService.Add")
        .description("Adds a new language to the system.")
        .response_bad_request()
        .response_conflict()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<AppState>,
    Json(request): Json<LanguageServiceAddRequest>,
) -> ApiResult<Json<LanguageServiceAddResponse>> {
    let form = request.form.try_into()?;
    let new_language_id = state.add_language(form).await?;

    let response_data = LanguageServiceAddResponse {
        language_id: new_language_id.into(),
    };
    Ok(Json(response_data))
}
