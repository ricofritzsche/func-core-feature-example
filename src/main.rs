pub mod features;
pub mod shared;

use dotenvy::dotenv;
use std::env;
use asset_tracking::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    run(&db_url).await
}
