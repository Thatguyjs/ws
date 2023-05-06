mod listener;
// mod threadpool;
mod http;

use http::{HttpServer, request::HttpMethod};
use clap::Parser;

use std::{path::Path, fs, io::ErrorKind, collections::HashMap};


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 8080, required = false)]
    port: u16,

    #[clap(short, long, default_value = "./src", required = false)]
    dir: String,

    #[clap(short, long, required = false, multiple_occurrences = true, number_of_values = 2)]
    redirect: Vec<String>
}


fn main() {
    let args = Args::parse();
    let mut redirects = HashMap::new();

    for redirect in args.redirect.chunks_exact(2) {
        let (mut from, to) = (redirect[0].clone(), redirect[1].clone());

        if !from.starts_with('/') {
            from.insert(0, '/');
        }

        redirects.insert(from, to);
    }

    let addr = format!("127.0.0.1:{}", args.port).parse().expect("Invalid port");
    let (server, shutdown) = HttpServer::bind(addr).expect("Failed to create HttpServer");

    ctrlc::set_handler(move || {
        if let Err(e) = shutdown.shutdown() {
            eprintln!("Failed to shutdown server: {}", e);
            std::process::exit(1);
        }
    }).expect("Failed to set Ctrl-C handler");

    println!("Server listening at 127.0.0.1:{}", args.port);

    server.listen(|req, mut res| {
        if req.method != HttpMethod::Get {
            res.set_status(405);

            if let Err(e) = res.send() {
                eprintln!("Failed to Send Response: {}", e);
            }

            return;
        }

        // Redirects
        if let Some(dest) = redirects.get(&req.path) {
            res.set_status(302).set_header("Location", dest);

            if let Err(e) = res.send() {
                eprintln!("Failed to Send Response: {}", e);
            }

            return;
        }

        // Get the correct path and MIME type
        let mut req_path = Path::new(&args.dir).join(&req.path[1..]);
        let res_mime: &str;

        match &req_path.extension() {
            Some(ext) => {
                res_mime = http::mime::mime_from_ext(ext.to_str().unwrap(), None);
            },
            _ => {
                req_path.push("index.html");
                res_mime = http::mime::mime_from_ext("html", None);
            }
        }

        match fs::read(req_path) {
            Ok(data) => {
                res.set_status(200)
                    .set_header("Content-Type", res_mime)
                    .set_body(&data);
            },
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                res.set_status(404)
                    .set_header("Content-Type", "text/plain")
                    .set_body(b"404 Not Found");
            },
            Err(e) => {
                eprintln!("File Read Error: {}", e);
            }
        }

        if let Err(e) = res.send() {
            eprintln!("Failed to Send Response: {}", e);
        }
    });
}
