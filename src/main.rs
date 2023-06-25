mod http;
mod listener;


fn main() {
    let opts = http::HttpOptions::parse_file(".ws").unwrap();

    // Parse config & start server, provide Ctrl-C handling
    let (server, mut shutdown) = http::HttpServer::bind("127.0.0.1:8080".parse().unwrap(), Some(opts)).unwrap();

    ctrlc::set_handler(move || {
        shutdown.shutdown().unwrap();
    }).expect("Failed to set Ctrl-C handler");

    server.run();
}
