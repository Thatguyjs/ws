mod config;
mod serve;

use config::ServerConfig;
use serve::ServeDir;
use axum::{Router, response::Redirect, routing::get};




#[tokio::main]
async fn main() {
    let config = ServerConfig::load().unwrap();

    let dir = ServeDir::new(config.dir);
    let mut app = Router::new()
        .fallback_service(dir);

    for (from, to) in config.redirects {
        app = app.route(&from, get(|| async {
            let to = to; // Prevent it from thinking 'to' won't live long enough
            Redirect::temporary(&to)
        }));
    }

    println!("Listening at {:?}", config.address);

    axum::Server::bind(&config.address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
