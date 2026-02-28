use axum::{
    routing::{delete, get, post, put},
    Router,
};
use axum::middleware as axum_middleware;
use sqlx::MySqlPool;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use axum::http::{HeaderValue, Method, header};
use tracing_subscriber::EnvFilter;

mod config;
mod db;
mod models;
mod routes;
mod middleware;
mod auth;
mod utils;

use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub config: Config,
    pub s3_client: aws_sdk_s3::Client,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env();
    let pool = db::create_pool(&config.database_url).await;

    let aws_config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&aws_config);

    let state = AppState {
        pool,
        config: config.clone(),
        s3_client,
    };

    let frontend_url = config.frontend_url.clone();
    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().expect("Invalid FRONTEND_URL"))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);

    let admin_router = Router::new()
        .route("/articles", get(routes::admin::articles::list_articles))
        .route("/articles", post(routes::admin::articles::create_article))
        .route("/articles/:id", get(routes::admin::articles::get_article_by_id))
        .route("/articles/:id", put(routes::admin::articles::update_article))
        .route("/articles/:id", delete(routes::admin::articles::delete_article))
        .route("/books", post(routes::admin::books::create_book))
        .route("/books/:id", put(routes::admin::books::update_book))
        .route("/books/:id", delete(routes::admin::books::delete_book))
        .route("/upload-image", post(routes::admin::upload::upload_image))
        .layer(axum_middleware::from_fn_with_state(state.clone(), middleware::auth::require_auth));

    let app = Router::new()
        .route("/health", get(routes::health::health))
        .route("/articles", get(routes::articles::list_articles))
        .route("/articles/:slug", get(routes::articles::get_article))
        .route("/books", get(routes::books::list_books))
        .route("/books/:slug", get(routes::books::get_book))
        .route("/login", post(routes::auth::login))
        .route("/logout", post(routes::auth::logout))
        .route("/me", get(routes::auth::me))
        .nest("/admin", admin_router)
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Backend listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
