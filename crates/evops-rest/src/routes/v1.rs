use aide::axum::ApiRouter;

use crate::AppState;

mod auth;
mod events;
mod languages;
mod tags;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .nest("/auth", self::auth::router())
        .nest("/events", self::events::router())
        .nest("/languages", self::languages::router())
        .nest("/tags", self::tags::router())
}
