use tonic::{Request, Response, Status};

use crate::AppState;
use crate::pb::language_service_server::{LanguageService, LanguageServiceServer};
use crate::pb::{LanguageServiceAddRequest, LanguageServiceAddResponse};

pub fn server(state: AppState) -> LanguageServiceServer<self::Service> {
    LanguageServiceServer::new(self::Service { state })
}

pub struct Service {
    #[allow(dead_code)]
    state: AppState,
}

#[tonic::async_trait]
impl LanguageService for self::Service {
    async fn add(
        &self,
        _request: Request<LanguageServiceAddRequest>,
    ) -> Result<Response<LanguageServiceAddResponse>, Status> {
        // let request_data = request.into_inner();

        // let form = {
        //     request_data
        //         .form
        //         .ok_or_else(|| {
        //             ApiError::InvalidArgument({
        //                 let err_msg = "LanguageServiceAddRequest.form must not be null.";
        //                 err_msg.to_owned()
        //             })
        //         })?
        //         .try_into()?
        // };
        // let new_language_id = self.state.add_language(form).await?;

        // let response_data = LanguageServiceAddResponse {
        //     language_id: new_language_id.into_inner().to_string(),
        // };
        // Ok(Response::new(response_data))

        Err(Status::unimplemented(""))
    }
}
