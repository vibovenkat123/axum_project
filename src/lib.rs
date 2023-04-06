use axum::{routing::get, Router};
mod handlers;
pub async fn start_server() -> Result<(), ()> {
    axum::Server::bind(&"0.0.0.0:7001".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
    Ok(())
}
fn app() -> Router {
    Router::new()
        .route("/ping", get(handlers::ping::start))
        .route("/echo/:msg", get(handlers::echo::start))
}
