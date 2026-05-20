pub mod email;
pub mod in_app;
pub mod sms;
use std::{ops::Deref, time::Duration};

use chrono::Utc;
use futures::{Stream, StreamExt};
use prost_types::Timestamp;
use tokio::{sync::mpsc, time::sleep};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};
use tracing::{info, warn};

use crate::{
    NotificationService, NotificationServiceInner, ResponseStream, ServiceResult,
    pb::notification::{SendRequest, SendResponse, send_request::Msg},
};

const CHANNEL_SIZE: usize = 1024;
pub trait Sender {
    fn send(
        self,
        svc: NotificationService,
    ) -> impl std::future::Future<Output = Result<SendResponse, Status>> + Send;
}

impl NotificationService {
    pub async fn process_send(
        &self,
        mut stream: impl Stream<Item = Result<SendRequest, tonic::Status>> + Send + 'static + Unpin,
    ) -> ServiceResult<ResponseStream> {
        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
        let notif = self.clone();
        tokio::spawn(async move {
            while let Some(Ok(req)) = stream.next().await {
                let notif_clone = notif.clone();
                let res = match req.msg {
                    Some(msg) => match msg {
                        Msg::Email(email) => email.send(notif_clone).await,
                        Msg::InApp(in_app) => in_app.send(notif_clone).await,
                        Msg::Sms(sms) => sms.send(notif_clone).await,
                    },
                    None => {
                        warn!("Invalid request");
                        Err(Status::invalid_argument("Invalid request"))
                    }
                };
                tx.send(res).await.unwrap();
            }
        });

        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }
}

fn to_ts() -> Timestamp {
    let now = Utc::now();
    Timestamp {
        seconds: now.timestamp(),
        nanos: now.timestamp_subsec_nanos() as i32,
    }
}

pub fn dummy_send() -> mpsc::Sender<Msg> {
    let (tx, mut rx) = mpsc::channel(CHANNEL_SIZE * 100);

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            info!("Sending message: {:?}", msg);
            sleep(Duration::from_millis(300)).await;
        }
    });
    tx
}

impl Deref for NotificationService {
    type Target = NotificationServiceInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tracing_subscriber::fmt::format::FmtSpan;

    use super::*;
    use crate::{
        AppConfig,
        pb::notification::{EmailMessage, InAppMessage, SmsMessage},
    };

    #[tokio::test]
    async fn send_should_work() -> Result<()> {
        tracing_subscriber::fmt()
            .with_span_events(FmtSpan::CLOSE)
            .with_env_filter("debug")
            .init();

        let config = AppConfig::load()?;
        let service = NotificationService::new(config);
        let stream = tokio_stream::iter(vec![
            Ok(EmailMessage::fake().into()),
            Ok(SmsMessage::fake().into()),
            Ok(InAppMessage::fake().into()),
        ]);

        let response = service.process_send(stream).await?;
        let ret = response.into_inner().collect::<Vec<_>>().await;
        assert_eq!(ret.len(), 3);

        Ok(())
    }
}
