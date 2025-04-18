use sqlx::PgPool;

pub async fn create_pool(url: &str) -> PgPool {
    PgPool::connect(url)
        .await
        .expect("Failed to connect to database")
}
