use http::Method;
use http::header::CONTENT_TYPE;
use tonic_reflection::server::v1::{ServerReflection, ServerReflectionServer};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

use evops_core::AppState;
use evops_models::{ApiError, ApiResult};

mod headers;
pub mod pb;
mod services;

pub fn router(state: &AppState) -> axum::Router {
    tonic::service::Routes::new(self::grpc_reflection_service())
        // server logic
        .add_service(crate::services::auth::server(state.arc_clone()))
        .add_service(crate::services::event::server(state.arc_clone()))
        .add_service(crate::services::language::server(state.arc_clone()))
        .add_service(crate::services::tag::server(state.arc_clone()))
        // auxiliary layers
        .into_axum_router()
        .layer(GrpcWebLayer::new())
        .layer(self::cors_layer())
}

fn grpc_reflection_service() -> ServerReflectionServer<impl ServerReflection> {
    tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(crate::pb::FILE_DESCRIPTOR_SET)
        .build_v1()
        .expect("reflection service should construct successfully")
}

fn cors_layer() -> CorsLayer {
    use self::headers::prelude::*;

    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST])
        .allow_headers([CONTENT_TYPE, GRPC_TIMEOUT, X_GRPC_WEB, X_USER_AGENT])
        .expose_headers([GRPC_STATUS, GRPC_MESSAGE, GRPC_STATUS_DETAILS_BIN])
}

fn auth<T>(state: &AppState, request: &tonic::Request<T>) -> ApiResult<evops_models::UserId> {
    let authorization_header_value = {
        request
            .metadata()
            .get(http::header::AUTHORIZATION.as_str())
            .ok_or_else(|| ApiError::Auth("No JWT access token provided.".to_owned()))?
            .to_str()
            .map_err(|e| ApiError::Auth(e.to_string()))?
    };
    let token = evops_models::JsonWebToken::new({
        authorization_header_value
            .strip_prefix("Bearer ")
            .ok_or_else(|| ApiError::Auth("Invalid JWT access token.".to_owned()))?
    });
    let user_id = state.decode_jwt(&token)?;
    Ok(user_id)
}
