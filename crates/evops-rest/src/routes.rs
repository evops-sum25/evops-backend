use aide::axum::ApiRouter;

use evops_core::AppState;

mod v1;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().nest("/v1", self::v1::router())
}
