use super::{HttpOptions, request::Request, response::*};
use std::{net::TcpStream, time::Duration, sync::Arc, path::PathBuf, fs};


fn mime_from_path(path: &PathBuf) -> Option<&'static str> {
    Some(match path.extension()?.to_str()? {
        "html" => "text/html",
        "htm" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "mjs" => "application/javascript",
        "wasm" => "application/wasm",

        "png" => "image/png",
        "jpeg" => "image/jpeg",
        "jpg" => "image/jpeg",
        "ico" => "image/x-icon",

        _ => return None
    })
}


pub struct Client {
    stream: TcpStream,
    options: Arc<HttpOptions>,
    open: bool
}

impl Client {
    pub fn new(stream: TcpStream, options: Arc<HttpOptions>) -> Self {
        Self {
            stream,
            options,
            open: true
        }
    }

    pub fn accept(&mut self) -> Option<Duration> {
        if !self.open {
            println!("Called accept() when closed!");
            return None;
        }

        match Request::from_stream(&self.stream) {
            Ok(mut request) => {
                if !request.path.rsplit_once('/').unwrap().1.contains('.') {
                    if !request.path.ends_with('/') {
                        request.path.push('/');
                    }
                    request.path.push_str(&self.options.index_file);
                }

                let mut path = self.options.directory.clone();
                path.push(&request.path[1..]);

                match fs::read(&path) {
                    Ok(body) => {
                        let mut response = Response::new(&mut self.stream, Status::Ok);

                        if let Some(mime) = mime_from_path(&path) {
                            response.set_header("Content-Type", mime);
                        }

                        let _ = response.send(&body);
                    },
                    Err(_) => {
                        let _ = Response::new(&mut self.stream, Status::NotFound).send(b"404 Not Found");
                    }
                }

                if request.headers.get("connection") == Some(&"keep-alive".into()) {
                    Some(self.options.keep_alive.clone())
                }
                else {
                    None
                }
            },

            _ => {
                self.open = false;
                None
            }
        }
    }

    pub fn is_closed(&self) -> bool {
        !self.open
    }
}
