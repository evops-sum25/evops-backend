use aide::axum::ApiRouter;

use crate::AppState;

mod events;
mod languages;
mod tags;
mod users;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .nest("/events", self::events::router())
        .nest("/languages", self::languages::router())
        .nest("/tags", self::tags::router())
        .nest("/users", self::users::router())
}
