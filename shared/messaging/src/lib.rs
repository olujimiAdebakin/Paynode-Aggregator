use async_nats::Client;
use anyhow::Result;

pub async fn connect_nats(url: &str) -> Result<Client> {
    let client = async_nats::connect(url).await?;
    Ok(client)
}
