use ntex::web;
use ntex::http;
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HttpError {
    pub msg: String,

    #[serde(skip)]
    pub status: http::StatusCode,
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.status, self.msg)
    }
}

impl std::error::Error for HttpError {}

impl web::WebResponseError for HttpError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        web::HttpResponse::build(self.status).json(&self)
    }
}