use tonic::{Request, Response, Status};

use crate::AppState;
use crate::pb::language_service_server::{LanguageService, LanguageServiceServer};
use crate::pb::{LanguageServiceAddRequest, LanguageServiceAddResponse};

pub fn server(state: AppState) -> LanguageServiceServer<self::Service> {
    LanguageServiceServer::new(self::Service { state })
}

pub struct Service {
    state: AppState,
}

#[tonic::async_trait]
impl LanguageService for self::Service {
    async fn add(
        &self,
        request: Request<LanguageServiceAddRequest>,
    ) -> Result<Response<LanguageServiceAddResponse>, Status> {
        todo!();
    }
}
