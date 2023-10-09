use actix_web::HttpServer;
use p_web::server;

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(move || {
        server::app(server::StorageType::Cookie).configure(crate::routes::user_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
