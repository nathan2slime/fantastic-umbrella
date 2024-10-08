pub mod auth;
pub mod health;
pub mod docs;

use ntex::web;

pub async fn default() -> web::HttpResponse {
    web::HttpResponse::NotFound().finish()
}