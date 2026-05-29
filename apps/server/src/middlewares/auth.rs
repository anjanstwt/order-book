use axum::{
    Extension, body::Body, http::Request, middleware::Next, response::Response as AxumResponse,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{Services, services::Response, types::AuthUser};

pub async fn auth(
    Extension(services): Extension<Services>,
    mut req: Request<Body>,
    next: Next,
) -> Result<AxumResponse, Response<()>> {
    let Some(auth_header) = req.headers().get("authorization") else {
        return Err(Response::not_authorized());
    };

    let Ok(auth_header) = auth_header.to_str() else {
        return Err(Response::not_authorized());
    };

    if !auth_header.starts_with("Bearer ") {
        return Err(Response::not_authorized());
    }

    let token = auth_header.strip_prefix("Bearer ").unwrap_or("");
    if token.is_empty() {
        return Err(Response::not_authorized());
    }

    let secret = services.env.auth_secret;

    let Ok(token_data) = decode::<AuthUser>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) else {
        return Err(Response::not_authorized());
    };

    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
