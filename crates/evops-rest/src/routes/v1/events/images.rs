use aide::axum::ApiRouter;

use crate::AppState;

mod _image_id;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().nest("/{image-id}", self::_image_id::router())
}
