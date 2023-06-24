use super::{HttpOptions, request::Request, response::*};
use std::{net::TcpStream, time::{Instant, Duration}, thread, sync::Arc, io, path::PathBuf, fs};


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
    id: usize,
    stream: TcpStream,
    started: Instant,
    timeout: Option<Duration>
}

impl Client {
    pub fn new(id: usize, stream: TcpStream, options: Arc<HttpOptions>) -> io::Result<Self> {
        let t_stream = stream.try_clone()?;

        thread::spawn(move || {
            let mut buf = [0u8; 1];

            loop {
                if let Ok(_) = t_stream.peek(&mut buf) {

                }
            }
        });

        Ok(Self {
            id,
            stream,
            started: Instant::now(),
            timeout: None
        })
    }

    pub fn set_timeout(&mut self, duration: Duration) {
        self.timeout = Some(duration);
    }

    fn handle_stream(options: Arc<HttpOptions>, mut stream: TcpStream) {
        match Request::from_stream(&stream) {
            Ok(mut request) => {
                if !request.path.rsplit_once('/').unwrap().1.contains('.') {
                    if !request.path.ends_with('/') {
                        request.path.push('/');
                    }
                    request.path.push_str(&options.index_file);
                }

                let mut path = options.directory.clone();
                path.push(&request.path[1..]);

                match fs::read(&path) {
                    Ok(body) => {
                        let mut response = Response::new(&mut stream, Status::Ok);

                        if let Some(mime) = mime_from_path(&path) {
                            response.set_header("Content-Type", mime);
                        }

                        let _ = response.send(&body);
                    },
                    Err(_) => {
                        let _ = Response::new(&mut stream, Status::NotFound).send(b"404 Not Found");
                    }
                }
            },
            _ => ()
        }
    }
}
