use axum::{routing::get, Router};

pub async fn start_server() -> Result<(), ()> {
    let app = Router::new().
        route("/ping", get(ping));
    axum::Server::bind(&"0.0.0.0:7001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
async fn ping() -> String {
    "pong".to_string()
}
