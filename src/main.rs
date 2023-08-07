mod config;

use config::ServerConfig;
use axum::Router;
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() {
    let config = ServerConfig::load().unwrap();

    let dir = ServeDir::new(config.dir);
    let app = Router::new().fallback_service(dir);

    println!("Listening at {:?}", config.address);

    axum::Server::bind(&config.address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
