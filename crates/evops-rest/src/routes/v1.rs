use aide::axum::ApiRouter;

use crate::AppState;

mod events;
mod tags;
mod users;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .nest("/events", self::events::router())
        .nest("/tags", self::tags::router())
        .nest("/users", self::users::router())
}
