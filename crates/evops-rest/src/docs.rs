use aide::transform::TransformOpenApi;
use schemars::SchemaGenerator;
use schemars::generate::SchemaSettings;
use strum::{Display, EnumMessage, IntoStaticStr, VariantArray};

use crate::DEFAULT_SECURITY_REQUIREMENT;

pub use self::routes::router;

mod routes;

#[derive(Display, IntoStaticStr, EnumMessage, VariantArray)]
#[allow(clippy::enum_variant_names)]
pub enum Tag {
    /// Manages login sessions
    AuthService,
    /// Manages event-related operations
    EventService,
    /// Manages human languages
    LanguageService,
    /// Manages event tags
    TagService,
}

pub fn use_openapi3_schemas() {
    aide::generate::in_context(|ctx| {
        ctx.schema = SchemaGenerator::new(SchemaSettings::openapi3());
    });
}

pub fn transform_api(mut api: TransformOpenApi) -> TransformOpenApi {
    for tag in self::Tag::VARIANTS {
        api = api.tag(aide::openapi::Tag {
            name: tag.to_string(),
            description: tag.get_documentation().map(ToOwned::to_owned),
            ..Default::default()
        });
    }

    api.title("evops.api").security_scheme(
        DEFAULT_SECURITY_REQUIREMENT,
        aide::openapi::SecurityScheme::Http {
            scheme: "bearer".to_owned(),
            bearer_format: Some("JWT".to_owned()),
            description: None,
            #[allow(clippy::default_trait_access)]
            extensions: Default::default(),
        },
    )
}
