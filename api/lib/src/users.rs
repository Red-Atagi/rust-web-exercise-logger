use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

async fn create_user() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))
    );
}