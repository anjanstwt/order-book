use crate::services::Response;

pub async fn health_check_controller() -> Response<()> {
    Response::success(None, Some("healthy".to_string()), None)
}
