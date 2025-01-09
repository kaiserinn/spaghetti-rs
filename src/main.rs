#[tokio::main]
async fn main() {
    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = sqlx::MySqlPool::connect(&db_url).await.unwrap();

    spaghetti_rs::run(pool).await;
}
