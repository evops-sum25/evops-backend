use aide::transform::TransformOpenApi;
use schemars::SchemaGenerator;
use schemars::generate::SchemaSettings;
use strum::{Display, EnumMessage, IntoStaticStr, VariantArray};

pub use self::routes::router;

mod routes;

#[derive(Display, IntoStaticStr, EnumMessage, VariantArray)]
pub enum Tag {
    /// CRUD operations on events
    EventService,
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

    api.title("evops.api")
}
