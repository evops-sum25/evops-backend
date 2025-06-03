use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
pub struct EventServiceCreateRequest {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, JsonSchema)]
pub struct EventServiceCreateResponse {
    pub name: String,
    pub description: String,
}
