use std::sync::Arc;

use aide::NoApi;
use aide::openapi::OpenApi;
use axum::Extension;

use evops_core::AppState;

type Result<T> = std::result::Result<T, self::error::Error>;

mod docs;
mod error;
mod routes;
mod types;

pub fn router(state: AppState) -> axum::Router {
    self::docs::use_openapi3_schemas();

    let mut api = NoApi(OpenApi::default());

    let api_routes = {
        self::routes::router()
            .with_state(state)
            .merge(self::docs::router())
            .finish_api_with(&mut api, self::docs::transform_api)
    };

    api_routes.layer(Extension(Arc::new(api)))
}
