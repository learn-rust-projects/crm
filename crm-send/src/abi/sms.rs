use tonic::Status;
use tracing::warn;

use super::{Sender, to_ts};
use crate::{
    NotificationService,
    pb::notification::{SendRequest, SendResponse, SmsMessage, send_request::Msg},
};

impl Sender for SmsMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::Sms(self)).await.map_err(|e| {
            warn!("Failed to send message: {:?}", e);
            Status::internal("Failed to send message")
        })?;
        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}

impl From<SmsMessage> for Msg {
    fn from(sms: SmsMessage) -> Self {
        Msg::Sms(sms)
    }
}

impl From<SmsMessage> for SendRequest {
    fn from(sms: SmsMessage) -> Self {
        let msg: Msg = sms.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(test)]
impl SmsMessage {
    pub fn fake() -> Self {
        use fake::{Fake, faker::phone_number::en::PhoneNumber};
        use uuid::Uuid;
        SmsMessage {
            message_id: Uuid::new_v4().to_string(),
            sender: PhoneNumber().fake(),
            recipients: vec![PhoneNumber().fake()],
            body: "Hello, world!".to_string(),
        }
    }
}
