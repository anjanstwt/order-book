use crate::services::Response;
use axum::Json;

pub async fn health_check_controller() -> Json<Response<()>> {
    Response::success(None, Some("healthy".to_string()), None).1
}
