use aide::axum::ApiRouter;

mod v1;

pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new().nest("/v1", self::v1::router())
}
