pub mod shared;
pub mod features;

use actix_web::{web, App, HttpServer};
use shared::db::create_pool;

use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn run(database_url: &str) -> std::io::Result<()> {
    let db_pool = create_pool(database_url).await;

    MIGRATOR.run(&db_pool).await.expect("Migration failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(features::init_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
