use p_web::server;

fn configurator(config: &mut actix_web::web::ServiceConfig) {
    crate::routes::user_config(config)
}

pub async fn run() -> std::io::Result<()> {
    server::run(server::SessionStore::Cookie, configurator).await
}
