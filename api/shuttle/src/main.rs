use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use api_lib::health::health;
use sqlx::Executor;


#[shuttle_runtime::main]
async fn main(
     #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    let pool = actix_web::web::Data::new(pool);
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool).service(health);
    };

    Ok(config.into())
}