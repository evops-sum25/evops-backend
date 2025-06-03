use aide::axum::ApiRouter;

use evops_core::AppState;

mod event;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().nest("/event", self::event::router())
}
