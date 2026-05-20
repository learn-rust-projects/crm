use anyhow::Result;
use crm::pb::crm::{
    RecallRequest, RemindRequest, WelcomeRequest,
    crm_server::{Crm, CrmServer},
};
use tonic::{Request, Response, Status, async_trait};

#[derive(Default)]
pub struct CrmServerImpl {}

#[async_trait]
impl Crm for CrmServerImpl {
    async fn welcome(
        &self,
        request: Request<WelcomeRequest>,
    ) -> Result<Response<crm::pb::crm::WelcomeResponse>, Status> {
        let input = request.into_inner();
        println!("welcome: {:?}", input);
        Ok(Response::new(crm::pb::crm::WelcomeResponse {
            id: format!("welcome-{}", input.id),
        }))
    }

    async fn recall(
        &self,
        request: Request<RecallRequest>,
    ) -> Result<Response<crm::pb::crm::RecallResponse>, Status> {
        let input = request.into_inner();
        println!("recall: {:?}", input);
        Ok(Response::new(crm::pb::crm::RecallResponse {
            id: format!("recall-{}", input.id),
        }))
    }

    async fn remind(
        &self,
        request: Request<RemindRequest>,
    ) -> Result<Response<crm::pb::crm::RemindResponse>, Status> {
        let input = request.into_inner();
        println!("remind: {:?}", input);
        Ok(Response::new(crm::pb::crm::RemindResponse {
            id: format!("remind-{}", input.id),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50000".parse().unwrap();
    let svc = CrmServerImpl::default();

    println!("CrmService listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(CrmServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
