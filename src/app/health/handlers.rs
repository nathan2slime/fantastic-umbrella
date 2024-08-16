use ntex::web;
use crate::app::health::dtos::HealthResponse;

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "HealthCheck", body = HealthResponse),
    ),
)]
#[web::get("/health")]
pub async fn get() -> impl web::Responder {
    web::HttpResponse::Ok().json(&HealthResponse {
        status: String::from("OK")
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}
