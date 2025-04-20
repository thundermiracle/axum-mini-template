use anyhow::{Ok, Result};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:4000")?;

    hc.do_get("/products").await?.print().await?;

    Ok(())
}
