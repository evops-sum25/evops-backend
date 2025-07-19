use aide::axum::ApiRouter;

use crate::AppState;

mod login;
mod refresh;
mod signup;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .nest("/login", self::login::router())
        .nest("/refresh", self::refresh::router())
        .nest("/signup", self::signup::router())
}
