mod user;

pub fn user_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        // services
        .service(
            actix_web::web::scope("/user")
                .service(user::create)
                .service(user::get),
        );
}
