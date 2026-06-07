use std::sync::Arc;

use axum::{
    Extension, body::Body, extract::Request, middleware::Next, response::Response as AxumResponse,
};
use database::user;
use jsonwebtoken::{DecodingKey, Validation, decode};
use sea_orm::{EntityTrait, SelectColumns};

use crate::{Services, services::Response, types::AuthUser};

pub async fn admin_auth(
    Extension(services): Extension<Arc<Services>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<AxumResponse, Response<()>> {
    let auth_header = req
        .headers()
        .get("authorization")
        .ok_or(Response::not_authorized())?
        .to_str()
        .map_err(|_e| Response::not_authorized())?;

    if !auth_header.starts_with("Bearer ") {
        return Err(Response::not_authorized());
    }

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(Response::not_authorized())?;

    if token.is_empty() {
        return Err(Response::not_authorized());
    }

    let secret = services.env.auth_secret.clone();

    let token_data = decode::<AuthUser>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_e| Response::not_authorized())?;

    // if !token_data.claims.is_admin {
    //     return Err(Response::not_authorized());
    // }
    //
    // // make a db call and check there
    // let user = database::user::Entity::find_by_id(token_data.claims.id)
    //     .select_column(user::Column::IsAdmin)
    //     .one(&services.db)
    //     .await
    //     .map_err(|_e| Response::not_authorized())?
    //     .ok_or(Response::not_authorized())?;
    //
    // if !user.is_admin {
    //     return Err(Response::not_authorized());
    // }

    if token_data.claims.email != "anjansuman80@gmail.com".to_string() {
        return Err(Response::not_authorized());
    }

    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
