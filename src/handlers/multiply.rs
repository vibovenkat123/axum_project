use axum::{
    extract::Path,
    response::Json,
};
use serde_json::{Value, json};

pub async fn start(Path((num1, num2)): Path<(i32, i32)>) -> Json<Value>{
    let product = num1 * num2;
    Json(json!({ "product": product }))
}
