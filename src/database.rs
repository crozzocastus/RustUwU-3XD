use actix_web::{web, App, HttpServer, Responder};
use crate::api::upload_file;
use sqlx::{MySqlPool, pool::PoolConnection, MySql};

#[derive(sqlx::FromRow)]
struct File {
    id: i32,
    name: String,
}

pub async fn handle_error(pool: &MySqlPool) {
    if let Err(e) = fetch_data(pool).await {
        eprintln!("Erro ao buscar dados: {}", e);
    }
}

pub async fn handle_connection() -> Result<PoolConnection<MySql>, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;
    let conn = pool.acquire().await?;
    Ok(conn)
}

pub async fn init_server() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool.");
    
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/files", web::get().to(get_files_html))
            .route("/upload", web::post().to(upload_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn fetch_data(pool: &MySqlPool) -> Result<Vec<File>, sqlx::Error> {
    let rows = sqlx::query_as!(File, "SELECT id, name FROM files")
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

pub async fn get_files_html(pool: web::Data<MySqlPool>) -> impl Responder {
    let files = fetch_data(pool.get_ref()).await.unwrap_or_default();
    
    let html = files.into_iter().map(|file| {
        format!("<p>ID: {} - Filename: {}</p>", file.id, file.name)
    }).collect::<Vec<String>>().join("");

    format!("<html><body>{}</body></html>", html)
}