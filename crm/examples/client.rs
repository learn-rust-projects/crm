use anyhow::Result;
use crm::pb::crm::{WelcomeRequest, crm_client::CrmClient};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = CrmClient::connect("http://[::1]:50000").await?;

    let request = tonic::Request::new(WelcomeRequest {
        id: "user-1".to_string(),
        interval: 7,
        content_ids: vec![1, 2, 3],
    });

    let response = client.welcome(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
