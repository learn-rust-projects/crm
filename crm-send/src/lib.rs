pub mod abi;
pub mod config;
pub mod pb;

#[allow(unused)]
#[derive(Clone)]
pub struct NotificationService {
    inner: Arc<NotificationServiceInner>,
}

#[allow(unused)]
pub struct NotificationServiceInner {
    config: AppConfig,
    sender: mpsc::Sender<Msg>,
}
use std::{pin::Pin, sync::Arc};

pub use config::AppConfig;
use futures::Stream;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status, Streaming, async_trait};

use crate::{
    abi::dummy_send,
    pb::notification::{
        SendRequest, SendResponse,
        notification_server::{Notification, NotificationServer},
        send_request::Msg,
    },
};
type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<SendResponse, Status>> + Send>>;

#[async_trait]
impl Notification for NotificationService {
    type SendStream = ResponseStream;

    async fn send(
        &self,
        request: Request<Streaming<SendRequest>>,
    ) -> ServiceResult<Self::SendStream> {
        let stream = request.into_inner();
        self.process_send(stream).await
    }
}

impl NotificationService {
    pub fn new(config: AppConfig) -> Self {
        let sender = dummy_send();
        let inner = NotificationServiceInner { config, sender };
        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn into_server(self) -> NotificationServer<Self> {
        NotificationServer::new(self)
    }
}
