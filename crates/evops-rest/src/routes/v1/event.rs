use aide::axum::ApiRouter;

use evops_core::AppState;

mod create;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().nest("/create", self::create::router())
}
