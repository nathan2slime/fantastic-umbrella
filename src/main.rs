mod app;
mod utils;

use ntex::web;
use app::{health, docs};


#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new().service(web::scope("/api/v1").configure(docs::handlers::config)
            .configure(health::handlers::config))
            .default_service(web::route().to(app::default))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

