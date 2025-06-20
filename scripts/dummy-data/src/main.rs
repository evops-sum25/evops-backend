use tonic::Request;

mod api;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let server_addr = "http://0.0.0.0:8080";
    let mut api = self::api::try_connect(server_addr).await?;

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
