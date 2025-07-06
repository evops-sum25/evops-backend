use std::sync::Arc;

use aide::NoApi;
use aide::openapi::OpenApi;
use axum::Extension;
use axum::http::StatusCode;
use axum::routing::options;

use evops_core::AppState;

mod docs;
mod error;
mod routes;
mod types;

pub fn router(state: AppState) -> axum::Router {
    self::docs::use_openapi3_schemas();

    let mut api = NoApi(OpenApi::default());

    let api_routes = {
        self::routes::router()
            .route(
                // HACK: I guess this shouldn't stay here forever...
                "/{*cors_preflight}",
                options(async || StatusCode::NO_CONTENT),
            )
            .with_state(state)
            .merge(self::docs::router())
            .finish_api_with(&mut api, self::docs::transform_api)
    };

    api_routes.layer(Extension(Arc::new(api)))
}
