use aide::axum::ApiRouter;

mod create;

pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new().nest("/create", self::create::router())
}
