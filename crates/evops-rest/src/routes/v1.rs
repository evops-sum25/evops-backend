use aide::axum::ApiRouter;

mod event;
mod user;

pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new()
        .nest("/event", self::event::router())
        .nest("/user", self::user::router())
}
