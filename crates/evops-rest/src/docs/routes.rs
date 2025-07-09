use std::sync::Arc;

use aide::NoApi;
use aide::axum::routing::get;
use aide::axum::{ApiRouter, IntoApiResponse};
use aide::openapi::OpenApi;
use aide::swagger::Swagger;
use axum::response::IntoResponse as _;
use axum::{Extension, Json, response::Redirect};
use tracing::error;

pub fn router() -> ApiRouter {
    if cfg!(debug_assertions) {
        self::inspect_openapi_errors();
    }

    ApiRouter::new()
        .route("/favicon.ico", {
            get(async || Redirect::temporary("https://cdn.svgporn.com/logos/swagger.svg"))
        })
        .route("/", get(async || Redirect::permanent("/swagger")))
        .route("/api.json", get(self::serve))
        .route("/swagger", {
            Swagger::new("/api.json")
                .with_title("evops.api - Swagger UI")
                .axum_route()
        })
        .route("/swagger/", get(async || Redirect::permanent("/swagger")))
}

fn inspect_openapi_errors() {
    tokio::spawn(async {
        aide::generate::on_error(|err| {
            error!("{err}");
        });
    });
}

async fn serve(Extension(api): Extension<Arc<NoApi<OpenApi>>>) -> impl IntoApiResponse {
    Json(api).into_response()
}
