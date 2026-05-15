use std::{ops::Deref, pin::Pin, sync::Arc};

use futures::Stream;
use sqlx::PgPool;
use tonic::{Response, Status};

use crate::{
    config::AppConfig,
    pb::user_stats::{
        QueryRequest, RawQueryRequest, User,
        user_stats_server::{UserStats, UserStatsServer},
    },
};

pub mod abi;
pub mod config;
pub mod pb;

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

#[derive(Clone)]
pub struct UserStatsService {
    inner: Arc<UserStatsServiceInner>,
}
#[allow(unused)]
pub struct UserStatsServiceInner {
    config: AppConfig,
    pool: PgPool,
}

impl UserStatsService {
    pub async fn new(config: AppConfig) -> Self {
        let pool = PgPool::connect(&config.server.db_url)
            .await
            .expect("Failed to connect to db");
        let inner = UserStatsServiceInner { config, pool };
        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn into_server(self) -> UserStatsServer<Self> {
        UserStatsServer::new(self)
    }
}

impl UserStats for UserStatsService {
    /// Server streaming response type for the Query method.
    type QueryStream = ResponseStream;

    /// Server streaming response type for the RawQuery method.
    type RawQueryStream = ResponseStream;

    #[allow(
        mismatched_lifetime_syntaxes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn raw_query<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<RawQueryRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = ServiceResult<Self::RawQueryStream>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let query = request.into_inner();
            self.raw_query(query).await
        })
    }

    #[allow(
        mismatched_lifetime_syntaxes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn query<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<QueryRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = ServiceResult<Self::QueryStream>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let query = request.into_inner();
            self.query(query).await
        })
    }
}

impl Deref for UserStatsService {
    type Target = UserStatsServiceInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
