use actix_web::{web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://127.0.0.1:9191");
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 9191))?
    .run()
    .await
}