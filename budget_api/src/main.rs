use axum::{
    Router,
    extract::{Json, State},
    routing::get};
use tokio::net::TcpListener;
use sqlx::{Pool, Sqlite, sqlite::SqlitePool};
use std::fs::File;
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize, Clone, sqlx::FromRow)]
struct ShopPurchaseListItem {
    shop_purchase_list_item_id: Option<i32>,
    name: String,
    is_purchased: bool,
}

async fn get_items(State(pool): State<SqlitePool>) -> Json<Vec<ShopPurchaseListItem>>{
    let db = sqlx::query_as::<_, ShopPurchaseListItem>("Select * FROM items").fetch_all(&pool).await.unwrap();
    Json(db)
}

async fn post_item(State(pool): State<SqlitePool>, Json(data): Json<ShopPurchaseListItem>) -> String {
    let name = &data.name;
    let is_purchased = data.is_purchased;
    sqlx::query("INSERT INTO items (name, is_purchased) VALUES (?, ?)")
        .bind(name).bind(is_purchased).execute(&pool).await.unwrap();
    format!("Added {} with status if purchased on {}.", name, is_purchased)
}

async fn init_db(db_name: &str) -> Pool<Sqlite>{
    if !Path::new(db_name).exists() {
        File::create(db_name).unwrap();
    }

    let db_url = format!("sqlite://{}", db_name);
    let pool = SqlitePool::connect(&db_url).await.unwrap();

    sqlx::query("CREATE TABLE IF NOT EXISTS items (shop_purchase_list_item_id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, is_purchased BOOLEAN NOT NULL)").execute(&pool).await.unwrap();

    pool
}

#[tokio::main]
async fn main() {
    let db_name = "purchased.db";
    let pool = init_db(db_name).await;

    let item_path  = "/items";
    let port_to_listen = "127.0.0.1:3000";

    let app = Router::new()
        .route(item_path, get(get_items).post(post_item))
        .with_state(pool);

    let listener = TcpListener::bind(port_to_listen).await.unwrap();

    println!("API server works on port: {}", port_to_listen);

    axum::serve(listener, app).await.unwrap();
}