use std::sync::Arc;

use chrono::{Duration, Utc};
use crm_metadata::pb::metadata::{Content, MaterializeRequest};
use crm_send::pb::notification::SendRequest;
use tonic::{Response, Status};
use user_stat::pb::user_stats::QueryRequest;

use crate::{
    CrmService,
    pb::crm::{
        RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest,
        WelcomeResponse,
    },
};
type ServiceResult<T> = Result<Response<T>, Status>;

impl CrmService {
    pub async fn welcome(&self, req: WelcomeRequest) -> ServiceResult<WelcomeResponse> {
        let request_id = req.id;
        let d1 = Utc::now() - Duration::days(req.interval as _);
        let d2 = d1 + Duration::days(1);
        let query = QueryRequest::new_with_dt("created_at", d1, d2);

        let res_user_stats = self.user_stats.clone().query(query).await?.into_inner();

        let contents = self
            .metadata
            .clone()
            .materialize(MaterializeRequest::new_with_ids(&req.content_ids))
            .await?
            .into_inner();

        let contents: Vec<Content> = contents
            .filter_map(|v: Result<Content, Status>| async move { v.ok() })
            .collect()
            .await;

        let contents = Arc::new(contents);

        // let (tx, rx) = mpsc::channel(1024);
        let sender = self.config.server.sender_email.clone();

        // tokio::spawn(async move {
        //     while let Some(Ok(user)) = res_user_stats.next().await {
        //         let contents = contents.clone();
        //         let sender = sender.clone();
        //         let tx: mpsc::Sender<SendRequest> = tx.clone();

        //         let req = SendRequest::new("Welcome".to_string(), sender,
        // &[user.email], &contents);         if let Err(e) = tx.send(req).await
        // {             warn!("Failed to send message: {:?}", e);
        //         }
        //     }
        // });
        // let reqs = ReceiverStream::new(rx);

        use futures::StreamExt;

        let reqs = res_user_stats.filter_map(move |item| {
            // filter_map里面是一个fnmut
            // 闭包，捕获变量后，变量所有权归闭包，使用时不能随便移出
            // 外层闭包只捕获了一次 sender 和 contents
            let sender = sender.clone();
            let contents = contents.clone();
            // 捕获后，这两个变量归闭包结构体所有
            // 你在闭包里只能 clone，不能移动所有权（因为是 FnMut，必须可重复调用）
            async move {
                let user = item.ok()?; // 过滤 Err

                let req = SendRequest::new(
                    "Welcome".to_string(),
                    sender.clone(),
                    &[user.email],
                    &contents.clone(),
                );

                Some(req)
            }
        });

        self.notification.clone().send(reqs).await?;

        Ok(Response::new(WelcomeResponse { id: request_id }))
    }

    pub async fn recall(&self, req: RecallRequest) -> ServiceResult<RecallResponse> {
        Ok(Response::new(RecallResponse {
            id: format!("recall-{}", req.id),
        }))
    }

    pub async fn remind(&self, req: RemindRequest) -> ServiceResult<RemindResponse> {
        Ok(Response::new(RemindResponse {
            id: format!("remind-{}", req.id),
        }))
    }
}
