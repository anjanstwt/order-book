use std::sync::Arc;

use axum::{Extension, Json, extract::State};
use serde::Deserialize;

use crate::{Services, services::Response, types::AuthUser};

#[derive(Deserialize)]
pub struct CreateMarketController {}

pub async fn create_market_controller(
    Extension(user): Extension<AuthUser>,
    State(services): State<Arc<Services>>,
    Json(body): Json<CreateMarketController>,
) -> Response<()> {

    

    Response::system_error()
}
