use std::sync::Arc;

use aide::openapi::OpenApi;
use aide::{NoApi, OperationIo};
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::routing::options;
use axum::{Extension, RequestPartsExt};

use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use evops_core::AppState;
use evops_models::ApiError;

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

const DEFAULT_SECURITY_REQUIREMENT: &str = "EvOps JWT";

#[derive(OperationIo)]
#[aide(
    // input_with = "TypedHeader<Authorization<Bearer>>",
    input,
)]
struct Auth(evops_models::UserId);

impl FromRequestParts<AppState> for self::Auth {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = {
            parts
                .extract::<TypedHeader<Authorization<Bearer>>>()
                .await
                .map_err(|e| ApiError::Auth(e.to_string()))?
        };
        let token = evops_models::JsonWebToken::new(bearer.token());
        let claims = state.decode_jwt(&token)?;
        Ok(Self(claims))
    }
}
