#[actix_web::main]
async fn main() -> std::io::Result<()> {
    p_user::startup::run().await
}
