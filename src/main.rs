use actix_web::{get, App, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::sync::Arc;

struct AppState {
    db_pool: PgPool,
}

#[get("/")]
async fn hello(data: actix_web::web::Data<Arc<AppState>>) -> impl Responder {
    let pool = &data.db_pool;
    println!("Hello world!!!");

    // Инкрементируем счетчик
    let result = sqlx::query(
        "INSERT INTO counter (id, count) VALUES (1, 1)
         ON CONFLICT (id) DO UPDATE SET count = counter.count + 1
         RETURNING count"
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(record) => {
            let count: i64 = record.get("count");
            format!("hello world, counter: {}", count)
        }
        Err(err) => {
            eprintln!("Database error: {}", err);
            "Internal Server Error".to_string()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Получаем URL базы данных из переменной окружения
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Создаем пул подключений к базе данных
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    // Создаем таблицу counter, если она не существует
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS counter (
            id INTEGER PRIMARY KEY,
            count BIGINT NOT NULL DEFAULT 0
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create counter table");

    // Создаем состояние приложения
    let app_state = Arc::new(AppState { db_pool: pool });

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(app_state.clone()))
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}