use axum::extract::Path;
pub async fn start(Path(msg): Path<String>) -> String {
    format!("{msg}")
}
