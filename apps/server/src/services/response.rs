use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response as AxumResponse},
};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Err {
    code: String,
    details: Option<String>,
}

impl Err {
    pub fn new(code: String, details: Option<String>) -> Self {
        Self { code, details }
    }
}

#[derive(Serialize)]
struct Meta {
    timestamp: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Response<T> {
    success: bool,
    #[serde(skip)]
    status_code: StatusCode,
    data: Option<T>,
    message: Option<String>,
    error: Option<Err>,
    meta: Meta,
}

impl<T> IntoResponse for Response<T>
where
    T: Serialize,
{
    fn into_response(self) -> AxumResponse {
        let status = self.status_code;
        (status, Json(self)).into_response()
    }
}

impl<T> Response<T> {
    fn new(
        success: bool,
        status_code: StatusCode,
        data: Option<T>,
        message: Option<String>,
        error: Option<Err>,
        meta: Meta,
    ) -> Self {
        Self {
            success,
            status_code,
            data,
            message,
            error,
            meta,
        }
    }

    pub fn success(
        data: Option<T>,
        message: Option<String>,
        status_code: Option<StatusCode>,
    ) -> Self {
        let code = status_code.unwrap_or(StatusCode::OK);
        let timestamp = Utc::now();

        Response::new(true, code, data, message, None, Meta { timestamp })
    }

    pub fn not_authorized() -> Self {
        let timestamp = Utc::now();
        Response::new(
            false,
            StatusCode::UNAUTHORIZED,
            None,
            Some("not authorized".to_string()),
            None,
            Meta { timestamp },
        )
    }

    pub fn error(status_code: StatusCode, message: Option<String>, error: Option<Err>) -> Self {
        Response::new(
            false,
            status_code,
            None,
            message,
            error,
            Meta {
                timestamp: Utc::now(),
            },
        )
    }

    pub fn system_error() -> Self {
        let timestamp = Utc::now();
        let code = StatusCode::INTERNAL_SERVER_ERROR;
        Response::new(
            false,
            code,
            None,
            Some("Internal server error".to_string()),
            None,
            Meta { timestamp },
        )
    }
}
