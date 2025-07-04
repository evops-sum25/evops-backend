use aide::axum::ApiRouter;

use crate::AppState;

mod _id;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().nest("/{id}", self::_id::router())
}
