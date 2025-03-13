
use anyhow::Result;
#[tokio::test]
async fn quick_dev() -> Result<()>{
    let hc = httpc_test::new_client("http://localhost")?;
    hc.do_get("hellow").await?.print().await?;
    Ok(())
}