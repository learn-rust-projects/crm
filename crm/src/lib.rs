pub mod abi;
pub mod config;
pub mod pb;

#[allow(unused)]
pub struct CrmService {
    config: AppConfig,
    user_stats: UserStatsClient<Channel>,
    notification: NotificationClient<Channel>,
    metadata: MetadataClient<Channel>,
}

pub use config::AppConfig;
use crm_metadata::pb::metadata::metadata_client::MetadataClient;
use crm_send::pb::notification::notification_client::NotificationClient;
use tonic::{Request, Response, Status, async_trait, transport::Channel};
use user_stat::pb::user_stats::user_stats_client::UserStatsClient;

use crate::pb::crm::{
    RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest, WelcomeResponse,
    crm_server::{Crm, CrmServer},
};
type ServiceResult<T> = Result<Response<T>, Status>;

#[async_trait]
impl Crm for CrmService {
    async fn welcome(&self, request: Request<WelcomeRequest>) -> ServiceResult<WelcomeResponse> {
        let req = request.into_inner();
        self.welcome(req).await
    }

    async fn recall(&self, request: Request<RecallRequest>) -> ServiceResult<RecallResponse> {
        let req = request.into_inner();
        self.recall(req).await
    }

    async fn remind(&self, request: Request<RemindRequest>) -> ServiceResult<RemindResponse> {
        let req = request.into_inner();
        self.remind(req).await
    }
}

impl CrmService {
    pub async fn try_new(config: AppConfig) -> anyhow::Result<Self> {
        Ok(CrmService {
            user_stats: UserStatsClient::connect(config.server.user_stats.clone()).await?,
            notification: NotificationClient::connect(config.server.notification.clone()).await?,
            metadata: MetadataClient::connect(config.server.metadata.clone()).await?,
            config,
        })
    }

    pub fn into_server(self) -> CrmServer<Self> {
        CrmServer::new(self)
    }
}
