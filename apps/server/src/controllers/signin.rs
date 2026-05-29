use std::{env, sync::Arc};

use axum::{Json, extract::State, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode, get_current_timestamp};
use sea_orm::{EntityTrait, Set, sea_query::OnConflict};
use serde::{Deserialize, Serialize};

use database::user;
use uuid::Uuid;

use crate::{Services, services::Response, types::AuthUser};

#[derive(Deserialize)]
pub struct SigninBody {
    name: String,
    email: String,
    image: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseData {
    user_id: Uuid,
    token: String,
}

#[axum::debug_handler]
pub async fn signin_controller(
    State(service): State<Arc<Services>>,
    Json(body): Json<SigninBody>,
) -> Response<ResponseData> {
    let Ok(user) = user::Entity::insert(user::ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(body.email),
        name: Set(body.name),
        image: Set(body.image),
        ..Default::default()
    })
    .on_conflict(
        OnConflict::column(user::Column::Email)
            .update_columns([user::Column::Name, user::Column::Image])
            .to_owned(),
    )
    .exec_with_returning(&service.db)
    .await
    else {
        return Response::system_error();
    };

    let secret = env::var("AUTH_SECRET").unwrap();

    let exp = get_current_timestamp() * 60 * 60 * 24 * 30;
    let auth = AuthUser::new(
        user.id,
        user.email,
        user.name,
        user.image,
        user.is_admin,
        exp,
    );

    let Ok(token) = encode(
        &Header::default(),
        &auth,
        &EncodingKey::from_secret(secret.as_ref()),
    ) else {
        return Response::system_error();
    };

    Response::success(
        Some(ResponseData {
            user_id: user.id,
            token: token,
        }),
        Some("You've successfully signed in".to_string()),
        Some(StatusCode::OK),
    )
}
