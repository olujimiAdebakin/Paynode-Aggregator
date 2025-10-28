use shared_database::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = connect_db().await?;
    println!("✅ Successfully connected to Neon DB!");
    Ok(())
}
