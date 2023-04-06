#[tokio::main]
async fn main() -> Result<(), ()> {
    axum_project::start_server().await?;
    Ok(())
}
