pub mod abi;
pub mod config;
pub mod pb;

#[allow(unused)]
pub struct MetadataService {
    config: AppConfig,
}

use std::pin::Pin;

pub use config::AppConfig;
use futures::Stream;
use tonic::{Request, Response, Status, Streaming, async_trait};

use crate::pb::metadata::{
    Content, MaterializeRequest,
    metadata_server::{Metadata, MetadataServer},
};
type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<Content, Status>> + Send>>;

#[async_trait]
impl Metadata for MetadataService {
    type MaterializeStream = ResponseStream;

    async fn materialize(
        &self,
        request: Request<Streaming<MaterializeRequest>>,
    ) -> ServiceResult<Self::MaterializeStream> {
        let query = request.into_inner();
        self.materialize(query).await
    }
}

impl MetadataService {
    pub fn new(config: AppConfig) -> Self {
        MetadataService { config }
    }

    pub fn into_server(self) -> MetadataServer<Self> {
        MetadataServer::new(self)
    }
}
