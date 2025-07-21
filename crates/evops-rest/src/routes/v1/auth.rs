use aide::axum::ApiRouter;

use crate::AppState;

mod login;
mod me;
mod refresh;
mod signup;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .nest("/login", self::login::router())
        .nest("/me", self::me::router())
        .nest("/refresh", self::refresh::router())
        .nest("/signup", self::signup::router())
}
