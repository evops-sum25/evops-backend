use aide::axum::ApiRouter;

mod event;
mod tag;
mod user;

pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new()
        .nest("/event", self::event::router())
        .nest("/tag", self::tag::router())
        .nest("/user", self::user::router())
}
