use tonic::Request;

use self::api::Api;

mod api;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let mut api = Api::try_connect("http://0.0.0.0:8080").await?;

    let _events = {
        api.event_service
            .list(Request::new(evops_pb::api::EventServiceListRequest {}))
            .await?
    };
    let _tags = {
        api.tag_service
            .list(Request::new(evops_pb::api::TagServiceListRequest {}))
            .await?
    };
    let _users = {
        api.user_service
            .list(Request::new(evops_pb::api::UserServiceListRequest {}))
            .await?
    };

    Ok(())
}
