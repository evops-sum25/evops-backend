use aide::axum::ApiRouter;

use crate::AppState;

mod v1;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().nest("/v1", self::v1::router())
}
