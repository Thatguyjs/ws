mod http;
mod listener;


fn main() {
    let config_file = http::HttpOptions::find_config();

    let opts = match config_file {
        Some(path) => {
            println!("Using config \"{}\" + CLI options", path.display());
            http::HttpOptions::parse_file(path)
                .expect("Failed to parse config file")
        },
        None => {
            println!("No config found. Fallback to defaults + CLI options");
            http::HttpOptions::default()
        }
    };

    // Parse config & start server, provide Ctrl-C handling
    let hosts = opts.hosts.clone();
    let (server, mut shutdown) = http::HttpServer::bind(hosts.as_slice(), opts).unwrap();

    ctrlc::set_handler(move || {
        shutdown.shutdown().unwrap();
    }).expect("Failed to set Ctrl-C handler");

    for host in hosts {
        println!("Listening at {host:?}");
    }

    server.run();
}
