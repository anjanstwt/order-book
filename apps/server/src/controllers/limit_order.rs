use crate::services::Response;

pub async fn limit_order_controller() -> Response<()> {
    Response::not_authorized()
}
