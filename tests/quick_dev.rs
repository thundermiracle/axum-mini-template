use anyhow::{Ok, Result};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:4000")?;

    hc.do_get("/hello-query?name=thundermiracle").await?.print().await?;
    hc.do_get("/hello-path/miracle/thunder").await?.print().await?;

    Ok(())
}
