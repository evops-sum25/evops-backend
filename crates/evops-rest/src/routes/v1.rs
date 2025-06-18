use aide::axum::ApiRouter;

mod events;
mod tags;
mod users;

pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new()
        .nest("/events", self::events::router())
        .nest("/tags", self::tags::router())
        .nest("/users", self::users::router())
}
