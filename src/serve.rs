// Service for serving static files

use axum::{http::{Request, Response}, body::Body};
use percent_encoding::percent_decode;
use tower::Service;
use std::{path::{PathBuf, Path}, convert::Infallible, pin::Pin, future::Future, task::Poll, fs, io::ErrorKind};


fn mime_from_path(path: &Path) -> Option<&str> {
    match path.extension()?.to_str()? {
        "htm" | "html" => Some("text/html"),
        "css" => Some("text/css"),
        "js" => Some("text/javascript"),
        "mjs" => Some("application/javascript"),

        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "ico" => Some("image/x-icon"),

        _ => None
    }
}


#[derive(Clone)]
pub struct ServeDir {
    path: PathBuf
}

impl ServeDir {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        ServeDir {
            path: path.as_ref().to_owned()
        }
    }
}

impl<B> Service<Request<B>> for ServeDir {
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    type Response = Response<Body>;

    fn poll_ready(&mut self, _: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let uri_path = match percent_decode(req.uri().path().trim_start_matches('/').as_ref()).decode_utf8() {
            Ok(p) => p,
            Err(_) => return Box::pin(async {
                Ok(Response::builder()
                    .status(400)
                    .body("400 Bad Request".into())
                    .unwrap())
            })
        };

        let mut path = PathBuf::from(&self.path);
        path.push(uri_path.as_ref());

        // Append 'index.html' to path
        if path.is_dir() {
            path.push("index.html");
        }

        let result = async move {
            Ok(match fs::read(&path) {
                Ok(data) => {
                    let mut res = Response::builder()
                        .status(200);

                    if let Some(mime) = mime_from_path(&path) {
                        res = res.header("Content-Type", mime);
                    }

                    res.body(data.into())
                        .expect("Failed to create 200 Response")
                },
                Err(e) if e.kind() == ErrorKind::NotFound => {
                    Response::builder()
                        .status(404)
                        .body("404 Not Found".into())
                        .expect("Failed to create 404 Response")
                },
                Err(e) => {
                    eprintln!("Error serving {path:?}: {e}");

                    Response::builder()
                        .status(500)
                        .body("500 Internal Server Error".into())
                        .expect("Failed to create 500 Response")
                }
            })
        };

        Box::pin(result)
    }
}
